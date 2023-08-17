#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/reference/frame-pallets/>
pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use sp_runtime::traits::MaybeDisplay;
	use sp_runtime::traits::AtLeast32Bit;	
	use frame_support::dispatch::fmt::Debug;
	use frame_support::{dispatch::DispatchResult, pallet_prelude::*};
	use frame_system::pallet_prelude::*;
	use frame_support::BoundedVec;
	use scale_info::prelude::string::String;
	
	#[derive(Encode, Decode, Debug, Clone, TypeInfo, PartialEq, MaxEncodedLen)]
	pub enum Access {
		External,
		InternalExternal,
	}

	impl Default for Access {
		fn default() -> Self {
			Access::External
		}
	}

	#[derive(Encode, Decode, Debug, Clone, Copy, TypeInfo, PartialEq, MaxEncodedLen)]
	pub enum Route {
		External = 0,
		Internal = 1,
	}
	#[pallet::pallet]
	pub struct Pallet<T>(_);

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

		/// Account index (aka nonce) type.
		type GameID: Parameter
			+ Member
			+ MaybeSerializeDeserialize
			+ Debug
			+ Default
			+ MaybeDisplay
			+ AtLeast32Bit
			+ Copy
			+ Encode
			+ Decode
			+ MaxEncodedLen;
	}

	pub(super) type Skey = BoundedVec<u8, ConstU32<256_u32>>;
	pub(super) type Sval = BoundedVec<u8, ConstU32<256_u32>>;
	pub(super) type DataEntry = (Skey,Sval);
	pub(super) type DataRecord = BoundedVec<DataEntry, ConstU32<256_u32>>;
	pub(super) type Permission<T> = (<T as self::Config>::GameID, Access);
	pub(super) type UserID<T> = (<T as self::Config>::GameID, <T as frame_system::Config>::AccountId);

	#[pallet::storage]
	pub(super) type WorldDataMap<T: Config> = StorageDoubleMap<_, Twox64Concat, T::GameID, Twox64Concat, Route, DataRecord, ValueQuery>;

	#[pallet::storage]
	pub(super) type UserDataMap<T: Config> = StorageDoubleMap<_, Twox64Concat, UserID<T>, Twox64Concat, Route, DataRecord, ValueQuery>;

	#[pallet::storage]
	pub(super) type AuthoritiesMap<T: Config> = StorageMap<_, Twox64Concat, T::AccountId, BoundedVec<Permission<T>, ConstU32<256_u32>>, ValueQuery>;

	// Pallets use events to inform users when important changes are made.
	// https://docs.substrate.io/main-docs/build/events-errors/
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		// [current level, who leveled up]
		LevelUp(u32, T::AccountId),

		// [Game world, updated data]
		WorldDataUpdate(T::GameID, DataEntry)
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
		
		InvalidAccess,

		NotFound,

		BadSize,

		Fake,

		/// When pushing has failed.
		BoundedVecOverflow,
	}

	#[pallet::genesis_config]
	pub struct GenesisConfig<T: Config> {
		pub fps_game_authority: Option<T::AccountId>,
		pub fps_game_id: T::GameID,
		pub platformer_game_authority: Option<T::AccountId>,
		pub platformer_game_id: T::GameID,
	}

	impl<T: Config> Default for GenesisConfig<T> {
		fn default() -> Self {
			Self { 
				fps_game_authority: Default::default(),
				fps_game_id: Default::default(),
				platformer_game_authority: Default::default(),
				platformer_game_id: Default::default()
			}
		}
	}

	#[pallet::genesis_build]
	impl<T: Config> BuildGenesisConfig for GenesisConfig<T> {
		fn build(&self) {

			if self.fps_game_authority.is_none() || self.platformer_game_authority.is_none()
			{
				//TODO: Log some error to notify about bad genesis config.
				return;
			}

			let fps_game_authority = self.fps_game_authority.clone().unwrap();
			let platformer_game_authority = self.platformer_game_authority.clone().unwrap();

			// FPS
			let mut fps_game_new_permissions = BoundedVec::<Permission<T>, ConstU32<256>>::default();
			fps_game_new_permissions.try_push((self.fps_game_id, Access::InternalExternal)).unwrap();// URGENT: FIX THIS. This is a panic hack.
			<AuthoritiesMap<T>>::insert(&fps_game_authority, fps_game_new_permissions);

			let entry : DataEntry = (
				String::into_bytes(String::from("Time")).try_into().unwrap(),
				1i32.to_le_bytes().to_vec().try_into().unwrap(),
			);

			let mut fps_game_new_data_record: DataRecord = DataRecord::default();
			let _ = fps_game_new_data_record.try_push(entry.clone());
			<WorldDataMap<T>>::insert(&self.fps_game_id, Route::External, fps_game_new_data_record);
			

			// PLATFORMER
			let mut platformer_game_new_permissions = BoundedVec::<Permission<T>, ConstU32<256>>::default();
			let _ = platformer_game_new_permissions.try_push((self.platformer_game_id, Access::InternalExternal)).unwrap();// URGENT: FIX THIS. This is a panic hack.
			<AuthoritiesMap<T>>::insert(&platformer_game_authority, platformer_game_new_permissions);

			let entry1 : DataEntry= (
				String::into_bytes(String::from("Kills")).try_into().unwrap(),
				0u32.to_le_bytes().to_vec().try_into().unwrap(),
			);
			let entry2 : DataEntry= (
				String::into_bytes(String::from("Deaths")).try_into().unwrap(),
				0u32.to_le_bytes().to_vec().try_into().unwrap(),
			);

			let mut platformer_game_new_data_record: DataRecord = DataRecord::default();
			let _ = platformer_game_new_data_record.try_push(entry1.clone());
			let _ = platformer_game_new_data_record.try_push(entry2.clone());
			<WorldDataMap<T>>::insert(&self.platformer_game_id, Route::External, platformer_game_new_data_record);
		}
	}

	fn is_authority<T: Config>(who : &T::AccountId, game : T::GameID) -> (bool, Access)
	{
		let bad_result = (false, Access::default());

		// Are they listed in authorities at all?
		let permissions = match <AuthoritiesMap<T>>::try_get(who)
		{
			Err(_) => {return bad_result;},
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

		bad_result
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

	fn is_authorized_call<T: Config> (origin: OriginFor<T>, game : T::GameID, route : Route) -> Result<T::AccountId, sp_runtime::DispatchError>
	{
		let who = ensure_signed(origin)?;

		let who_auth = is_authority::<T>(&who, game);
		
		ensure!(who_auth.0, Error::<T>::InvalidAuthority);

		if route == Route::Internal
		{
			ensure!(who_auth.1 == Access::InternalExternal, Error::<T>::InvalidAccess)
		}

		Ok(who)
	}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::call_index(0)]
		#[pallet::weight({0})]
		pub fn world_remove_data_record(origin: OriginFor<T>, game : T::GameID, entry : DataEntry, route : Route) -> DispatchResult
		{
			is_authorized_call::<T>(origin, game, route)?;
			
			ensure!(<WorldDataMap<T>>::contains_key(game, route), Error::<T>::NotFound);
			
			<WorldDataMap<T>>::mutate(game, route, |x| { 
				match x.iter().position(|cur_entry| cur_entry.0 == entry.0) {
					None => Err(Error::<T>::NotFound)?,	
					Some(d) => { 
						x.swap_remove(d); 
						Ok(())
					}
				}
			})
		}

		#[pallet::call_index(1)]
		#[pallet::weight({0})]
		pub fn world_update_data_record(origin: OriginFor<T>, game : T::GameID, entry : DataEntry, route : Route) -> DispatchResult
		{
			is_authorized_call::<T>(origin, game, route)?;

			if ! <WorldDataMap<T>>::contains_key(game, route)
			{
				let mut new_data_record: DataRecord = DataRecord::default();
				new_data_record.try_push(entry.clone()).map_err(|_| Error::<T>::BoundedVecOverflow)?;
				
				<WorldDataMap<T>>::insert(game, route, new_data_record);
			}
			else
			{
				<WorldDataMap<T>>::mutate(game, route, |record| {

					match record.iter().position(|cur_entry| cur_entry.0 == (&entry).0) {
						
						// Generate a new entry.
						None => { 
							record.try_push(entry.clone()).unwrap();// URGENT: FIX THIS. This is a panic hack.
						},
						
						// Assign new value to existing entry.
						Some(index) => { record[index].1 = (&entry).1.clone(); }
					};

				});
			}

			// Emit an event.
			Self::deposit_event(Event::WorldDataUpdate(game, entry));
			
			Ok(())
		}

		#[pallet::call_index(2)]
		#[pallet::weight({0})]
		pub fn world_mod_data_record(origin: OriginFor<T>, game : T::GameID, new_entry : DataEntry, route : Route, ) -> DispatchResult
		{
			is_authorized_call::<T>(origin, game, route)?;

			ensure!(<WorldDataMap<T>>::contains_key(game, route), Error::<T>::NotFound);
			
			<WorldDataMap<T>>::mutate(game, route, |record :&mut DataRecord| {

				if false //HACK. IDK why I need this. Compiler compaints.
				{
					return Err(Error::<T>::Fake);
				}
				
				let index : usize = record.iter().position(|cur_entry : &DataEntry| cur_entry.0 == (&new_entry).0).ok_or(Error::<T>::NotFound)?;
				
				let exisiting_entry:&mut DataEntry = &mut record[index];

				//Ensure same length
				const SUPPORTED_BYTE_LENGTH : usize = 4;
				ensure!(exisiting_entry.1.len() == SUPPORTED_BYTE_LENGTH, Error::<T>::BadSize);
				ensure!(new_entry.1.len() == SUPPORTED_BYTE_LENGTH, Error::<T>::BadSize);
				
				// Interpret existing numeric value.
				let existing_numeric : i32;
				{
					let mut vec_data : [u8; SUPPORTED_BYTE_LENGTH] = Default::default();
					vec_data.copy_from_slice(&exisiting_entry.1[0..SUPPORTED_BYTE_LENGTH]);
					existing_numeric = i32::from_le_bytes(vec_data);
				}

				// Interpret incoming numberic value.
				let incoming_numeric : i32;
				{
					let mut vec_data : [u8; SUPPORTED_BYTE_LENGTH] = Default::default();
					vec_data.copy_from_slice(&new_entry.1[0..SUPPORTED_BYTE_LENGTH]);
					incoming_numeric = i32::from_le_bytes(vec_data);
				}				

				// Increment|Decrement value.
				let result_numeric = existing_numeric + incoming_numeric;
				let result_numeric = result_numeric.to_le_bytes();

				exisiting_entry.1 = Sval::truncate_from(result_numeric.into()); 

				Ok(())
			})?;

			// // Emit an event.
			// Self::deposit_event(Event::WorldDataUpdate(game, new_entry));
			
			Ok(())
		}

		#[pallet::call_index(3)]
		#[pallet::weight({0})]
		pub fn user_remove_data_record(origin: OriginFor<T>, game : T::GameID, user : T::AccountId, route : Route, entry_key : Skey) -> DispatchResult
		{
			is_authorized_call::<T>(origin, game, route)?;
			
			let map_key = (game, user);

			ensure!(<UserDataMap<T>>::contains_key(&map_key, route), Error::<T>::NotFound);
			
			<UserDataMap<T>>::mutate(&map_key, route, |x| { 
				match x.iter().position(|cur_entry| cur_entry.0 == entry_key) {
					None => Err(Error::<T>::NotFound)?,	
					Some(d) => { 
						x.swap_remove(d); 
						Ok(())
					}
				}
			})
		}

		#[pallet::call_index(4)]
		#[pallet::weight({0})]
		pub fn user_update_data_record(origin: OriginFor<T>, game : T::GameID, user : T::AccountId, route : Route, entry : DataEntry) -> DispatchResult
		{
			is_authorized_call::<T>(origin, game, route)?;

			let map_key = (game, user);

			if ! <UserDataMap<T>>::contains_key(&map_key, route)
			{
				let mut new_data_record: DataRecord = DataRecord::default();
				new_data_record.try_push(entry.clone()).map_err(|_| Error::<T>::BoundedVecOverflow)?;

				<UserDataMap<T>>::insert(&map_key, route, new_data_record);
			}
			else
			{
				<UserDataMap<T>>::mutate(&map_key, route, |x| { 
					match x.iter().position(|cur_entry| cur_entry.0 == entry.0) {
						None => { let _ = x.try_push(entry); }, //HACK: URGENT: FIX THIS. This is a panic hack.
						Some(d) => { x[d].1 = entry.1; }
					};
				});
			}

			Ok(())
		}

		#[pallet::call_index(5)]
		#[pallet::weight({0})]
		pub fn register_game(origin: OriginFor<T>, game : T::GameID) -> DispatchResult
		{
			let who = ensure_signed(origin)?;

			frame_support::ensure!(! game_exists::<T>(&game), Error::<T>::AlreadyRegisteredGame);
			
			// For the registring user, access the permissions
			let new_entry:Permission<T> = (game, Access::InternalExternal);
			
			// When a user first registers a game, they don't exist in the AuthoritiesMap. Add them.
			if <AuthoritiesMap<T>>::contains_key(&who)
			{
				//TODO: use try_mutate.
				<AuthoritiesMap<T>>::mutate(&who, |x| {
					let _ = x.try_push(new_entry); // HACK: URGENT: FIX THIS. This is a panic hack.
				});
			}
			else
			{
				let mut new_permissions = BoundedVec::<Permission<T>, ConstU32<256>>::default(); 
				new_permissions.try_push(new_entry).map_err(|_| Error::<T>::BoundedVecOverflow)?;

				<AuthoritiesMap<T>>::insert(&who, new_permissions);
			};

			Ok(())
		}

		#[pallet::call_index(6)]
		#[pallet::weight({0})]
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
						x.try_push(entry).unwrap();// URGENT: FIX THIS. This is a panic hack.
					});
				},

				// Create entry.
				Err(_) => {
					let new_entry = (game, access);
					
					let mut new_permissions = BoundedVec::<Permission<T>, ConstU32<256>>::default(); 
					new_permissions.try_push(new_entry).map_err(|_| Error::<T>::BoundedVecOverflow)?;

					<AuthoritiesMap<T>>::insert(&new_authority, new_permissions);
				}
			};

			Ok(())
		}

		#[pallet::call_index(7)]
		#[pallet::weight({0})]
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
