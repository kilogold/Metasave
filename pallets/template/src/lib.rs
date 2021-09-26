#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/v3/runtime/frame>
pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

#[frame_support::pallet]
pub mod pallet {
	use sp_runtime::traits::MaybeDisplay;
	use sp_runtime::traits::AtLeast32Bit;	
	use frame_support::dispatch::fmt::Debug;
	use frame_support::{dispatch::DispatchResult, pallet_prelude::*};
	use frame_system::pallet_prelude::*;
	use sp_std::prelude::*;

	#[derive(Encode, Decode, Debug, Clone, PartialEq)]
	pub enum Access {
		External,
		InternalExternal,
	}

	impl Default for Access {
		fn default() -> Self {
			Access::External
		}
	}

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

		/// Account index (aka nonce) type. This stores the number of previous transactions
		/// associated with a sender account.
		type GameID: Parameter
			+ Member
			+ MaybeSerializeDeserialize
			+ Debug
			+ Default
			+ MaybeDisplay
			+ AtLeast32Bit
			+ Copy
			+ Encode
			+ Decode;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	// The pallet's runtime storage items.
	// https://docs.substrate.io/v3/runtime/storage
	#[pallet::storage]
	#[pallet::getter(fn something)]
	// Learn more about declaring storage items:
	// https://docs.substrate.io/v3/runtime/storage#declaring-storage-items
	pub type Something<T> = StorageValue<_, u32>;

	#[pallet::storage]
	#[pallet::getter(fn score)]
	pub(super) type Score<T> = StorageValue<_, u32>;

	type Skey = Vec<u8>;
	type Sval = Vec<u8>;
	type DataEntry = (Skey,Sval);
	type DataRecord = Vec<DataEntry>;
	type Permission<T> = (<T as self::Config>::GameID, Access);
	// pub(super) type GameAccount<T:Config> = (T::GameID, T::AccountId);

	#[pallet::storage]
	pub(super) type WorldDataExternalMap<T: Config> = StorageMap<_, Twox64Concat, T::GameID, DataRecord, ValueQuery>;

	// #[pallet::storage]
	// pub(super) type WorldDataInternalMap<T: Config> = StorageMap<_, Twox64Concat, T::GameID, DataRecord, ValueQuery>;

	// #[pallet::storage]
	// pub(super) type UserDataInternalMap<T: Config> = StorageMap<_, Twox64Concat, GameAccount<T>, DataRecord, ValueQuery>;

	// #[pallet::storage]
	// pub(super) type UserDataExternalMap<T: Config> = StorageMap<_, Twox64Concat, GameAccount<T>, DataRecord, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn authorities_map)]
	pub(super) type AuthoritiesMap<T: Config> = StorageMap<_, Twox64Concat, T::AccountId, Vec<Permission<T>>, ValueQuery>;

	// Pallets use events to inform users when important changes are made.
	// https://docs.substrate.io/v3/runtime/events-and-errors
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Event documentation should end with an array that provides descriptive names for event
		/// parameters. [something, who]
		SomethingStored(u32, T::AccountId),

		// [current level, who leveled up]
		LevelUp(u32, T::AccountId)
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// Error names should be descriptive.
		NoneValue,
		/// Errors should have helpful documentation associated with them.
		StorageOverflow,

		AlreadyRegisteredGame,

		AlreadyRegisteredAuthority,

		InvalidAuthority,
	}

	fn is_authority<T: Config>(who : &T::AccountId, game : T::GameID) -> (bool, Access)
	{
		let badResult = (false, Access::default());

		// Are they listed in authorities at all?
		let permissions = match <AuthoritiesMap<T>>::try_get(who)
		{
			Err(e) => {return badResult;},
			Ok(p) => p
		};

		// Do they have authority for the specified game?
		for p in permissions
		{
			if p.0 == game
			{
				return (true, p.1);
			}
		}

		badResult
	}

	fn game_exists<T: Config>(game : &T::GameID) -> bool
	{
		for authority in <AuthoritiesMap<T>>::iter()
		{
			let permissions = authority.1;

			if permissions.iter().any(|e| e.0 == *game)
			{
				return true;
			}
		}

		false
	}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// An example dispatchable that takes a singles value as a parameter, writes the value to
		/// storage and emits an event. This function must be dispatched by a signed extrinsic.
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn do_something(origin: OriginFor<T>, something: u32) -> DispatchResult {
			// Check that the extrinsic was signed and get the signer.
			// This function will return an error if the extrinsic is not signed.
			// https://docs.substrate.io/v3/runtime/origins
			let who = ensure_signed(origin)?;

			// Update storage.
			<Something<T>>::put(something);

			// Emit an event.
			Self::deposit_event(Event::SomethingStored(something, who));
			// Return a successful DispatchResultWithPostInfo
			Ok(())
		}

		/// An example dispatchable that may throw a custom error.
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1))]
		pub fn cause_error(origin: OriginFor<T>) -> DispatchResult {
			let _who = ensure_signed(origin)?;

			// Read a value from storage.
			match <Something<T>>::get() {
				// Return an error if the value has not been set.
				None => Err(Error::<T>::NoneValue)?,
				Some(old) => {
					// Increment the value read from storage; will error in the event of overflow.
					let new = old.checked_add(1).ok_or(Error::<T>::StorageOverflow)?;
					// Update the value in storage with the incremented result.
					<Something<T>>::put(new);
					Ok(())
				},
			}
		}	

		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn increment_score(origin: OriginFor<T>) -> DispatchResult {
			// Check that the extrinsic was signed and get the signer.
			// This function will return an error if the extrinsic is not signed.
			// https://substrate.dev/docs/en/knowledgebase/runtime/origin
			let who = ensure_signed(origin)?;

			let new_score;

			// Read a value from storage.
			match <Score<T>>::get() {
				None => {
					// Update the value with initial score point.
					new_score = 1;	
				}

				Some(old) => {
					// Increment the value read from storage; will error in the event of overflow.
					new_score = old.checked_add(1).ok_or(Error::<T>::StorageOverflow)?;
				},
			}

			// Update the value in storage with the incremented result.
			<Score<T>>::put(new_score);

			Self::deposit_event(Event::LevelUp(new_score, who));

			Ok(())		
		}

		#[pallet::weight(10_000)]
		pub fn register_game(origin: OriginFor<T>, game : T::GameID) -> DispatchResult
		{
			let who = ensure_signed(origin)?;

			frame_support::ensure!(! game_exists::<T>(&game), Error::<T>::AlreadyRegisteredGame);

			let new_entry = (game, Access::InternalExternal);
			<AuthoritiesMap<T>>::insert(&who, vec!(new_entry));

			Ok(())
		}

		#[pallet::weight(10_000)]
		pub fn add_authority(origin: OriginFor<T>, game : T::GameID, new_authority : T::AccountId, access : Access) -> DispatchResult
		{			
			let who = ensure_signed(origin)?;

			// Ensure only authorities add other authorities.
			let who_auth = is_authority::<T>(&who, game);
			frame_support::ensure!(who_auth.0, Error::<T>::InvalidAuthority);

			// Ensure new authority is not already registered
			let new_auth = is_authority::<T>(&new_authority, game);
			frame_support::ensure!(!new_auth.0, Error::<T>::InvalidAuthority);

			// Does new authority have a map entry?
			match <AuthoritiesMap<T>>::try_get(new_authority.clone()) //Hack: workaround for borrowing in match arms.
			{
				// Update entry.
				Ok(_) => {
					<AuthoritiesMap<T>>::mutate(&new_authority, |x| {
						let entry = (game,access);
						x.push(entry);
					});
				},

				// Create entry.
				Err(_) => {
					let new_entry = (game, access);
					<AuthoritiesMap<T>>::insert(&new_authority, vec!(new_entry));
				}
			};

			Ok(())
		}

		#[pallet::weight(10_000)]
		pub fn remove_authority(origin: OriginFor<T>, game : T::GameID, removed_authority : T::AccountId) -> DispatchResult
		{			
			let who = ensure_signed(origin)?;

			// Ensure only authorities remove authorities.
			let who_auth = is_authority::<T>(&who, game);
			frame_support::ensure!(who_auth.0, Error::<T>::InvalidAuthority);

			// If removing auhtority other than self...
			if who != removed_authority
			{
				// Ensure removed authority is already registered
				let new_auth = is_authority::<T>(&removed_authority, game);
				frame_support::ensure!(new_auth.0, Error::<T>::InvalidAuthority);
			}

			// Mutate for removal & return Result. We should not error here at this point.
			<AuthoritiesMap<T>>::mutate(&removed_authority, |x| {
				let index = x.iter().position(|p| p.0 == game).ok_or(Error::<T>::InvalidAuthority)?;
				x.swap_remove(index); 
				Ok(())
			})
		}
	}
}
