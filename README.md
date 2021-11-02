# Metasave
This project was developed for the [Encode Polkadot Hackathon](https://www.encode.club/polkadot-club-hackathon). 
## Video Demo
(click to play)
[![image](https://user-images.githubusercontent.com/1028926/139475111-90bc8c40-ef4a-4c10-a520-7ff8d3468668.png)](https://www.youtube.com/watch?v=oZd8Vu2ZqiQ)

The following are the resources illustrated in the video:
* [Platformer Game](https://dev.azure.com/bonillakelvin/MetaSave/_git/Polkadot_Platformer)
* [FPS Game](https://dev.azure.com/bonillakelvin/MetaSave/_git/Polkadot_FPS)

## Concept
Metasave is a data storage protocol for game worlds. The main goal of Metasave is to provide a mechanism where the consequences of one game carry over to any other game in real-time. Using permissionless read access, any game can react to events ocurring in any other game world irrespective of genre, platform, or hardware. To illustrate the concept, we will explore a hypothetical implementation of Metasave involving two popular games at the time of writing: 
* [Monster Hunter World](http://www.monsterhunterworld.com/us/)
* [Rainbow Six Siege](https://www.ubisoft.com/en-gb/game/rainbow-six/siege)

![image](https://user-images.githubusercontent.com/1028926/138626583-67dbe1a2-6991-43bc-bfd9-f45712ea6b8e.png)

## Architecture
### World Data Map (StorageDoubleMap)
![image](https://user-images.githubusercontent.com/1028926/138798123-5ba5866e-e222-4d33-a4a8-facd31159213.png)  
**DataEntry** Keys and Values can be stored as any arbitrary data by virtue of a byte vector. This allows developers to choose and optimize world data to their desired use case. We could use a simple character string as a key, or we could use any arbitrary complex object represented in binary format.
### Authority Map (StorageMap)
![image](https://user-images.githubusercontent.com/1028926/138798404-5e994e26-8d95-4a24-a150-d7ea4717105c.png)  
An account is tied to a **Permission** which is evaluated with every storage update transaction. Game world data can only be modified by an authority (an account with permissions for said game). Each world data is partitioned into two categories:
* External
* Internal

The difference between the two are purely semantic. These categories act as a means to distinguish which accounts have Write-Access to world data. Read-Access is available to everyone.

### Route vs Access  
The similarity between **Access** and **Route** enums is intentional. The operative difference is that **Access** designates permission, whereas **Route** indicates intent. You can think of is as:

> "I want to update [*internal*] data for GAME1, but I only have [*external*] access."

### Example Usage
Here's what the order of events look like to begin using Metasave:  
![image](https://user-images.githubusercontent.com/1028926/138800337-314bd68c-d573-46a2-b71c-dfa341920ce9.png)  
After this sequence of events, **AccountA** & **AccountB** may update the World1 calendar at any time. World2 will automatically receive this update and respond accordingly. 
Neither developer was required to implement custom logic for interoperability. Everyone can publicly see the data and freely subscribe to changes from their own world and anyone else's.  

Note **AccountC** has no authority to modify World1 data, thus World1 is protected. **AccountA/AccountB** (dev team) may grant **External** access to **AccountC** in order to write save data on World1's game, while still protecting World1's **Internal** data from anyone outside the dev team. 


## WIP Features
### Individual On-Chain User Data
![image](https://user-images.githubusercontent.com/1028926/139700828-55fd4e44-3eb1-4b1c-af8d-83558fa4cc32.png)  
User data is leveraged in a similar manner as world data, except it pertains to each individual user. This opens up novel use cases, such as anti-cheat & verifiable game progress. Other extraordinary implementations may include IoT.

#### Vending Machine IoT Example
In the diagram below, we explore how to bring [Final Fantasy VII Remake](https://www.playstation.com/en-us/games/final-fantasy-vii-remake/)'s vending machines into real life. For the purposes of this example, we are using a 
![image](https://user-images.githubusercontent.com/1028926/139705304-0e95736c-9843-4a55-ac29-c593a15854f2.png)
**Why not just tokenize GIL (like an ERC-20)?**  
While tokenization is a valid (and sometimes ideal) strategy, it needs to be carefully considered...  
Tokenization often leads to relinquishing control of asset utility, which can adversely affect game design & balance. By restricting the data to a player's save file, the game designer is able to retain the quality control of their game's experience (the token asset doesn't run rampant through DeFi protocols). Furthermore, tokens alone are not enough to provide bespoke verification. In the example above, it is not enough to be able to afford the item. As a player, you must also progress far enough into the game in order to qualify by defeating Bahamut. The only way to accomplish this in a tokenized world is to mint yet another token (like an achievement) to track the progress. Ultimately, we end up juggling many tokens for a single game, which may be costly to transfer to other wallets/accounts, and/or also introduce many game-explot vectors (or even facilitate an unintended pay-2-win model via trading achievement tokens).

# Tech Specs
## Dependencies
*  Extended version of the [Substrate .Net API](https://github.com/ajuna-network/SubstrateNetApi), already included within the game demos listed above.
*  Unity3D 2019.4

## Deployment
*  Follow typical [node template](https://github.com/substrate-developer-hub/substrate-node-template) deployment (pallet is included and installed).
*  Clone game demos linked above.
*  Open game demos with Unity and play.

## Notes
*  Metasave is preconfigured to have default Alice & Bob accounts registered with their respective games (see video for Alice & Bob setup). You may find the preconfiguration in the pallet's [lib.rs](https://github.com/kilogold/HackWeek-Sept2021/blob/daba356a66f1b5115c699543270c48332e3b2db4/pallets/template/src/lib.rs#L148).
