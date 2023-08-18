![image](https://github.com/kilogold/Metasave/assets/1028926/c76e18b2-b18c-4c32-9d69-724b0f9b2a4f)

![image](https://github.com/kilogold/Metasave/assets/1028926/91415c13-35b7-4556-ac3c-19789c94587a)

# Metasave
## Video Intro 
(click to play)
[![image](https://user-images.githubusercontent.com/1028926/141672480-4ab24560-a71d-4fe2-bf7f-845a9053e0af.png)](https://www.youtube.com/watch?v=rfkjQQ0yccw)

## Product Presentation
(click to play)
[![image](https://github.com/kilogold/Metasave/assets/1028926/fc7c30e4-927a-43b4-a5cc-7ccf31be446d)](https://youtu.be/42r33VnvF3Q)

## Video Tutorial
(click to play)
[![image](https://user-images.githubusercontent.com/1028926/139475111-90bc8c40-ef4a-4c10-a520-7ff8d3468668.png)](https://www.youtube.com/watch?v=oZd8Vu2ZqiQ)

The following are the resources illustrated in the video:
* [Platformer Game](https://dev.azure.com/bonillakelvin/MetaSave/_git/Polkadot_Platformer)
* [FPS Game](https://dev.azure.com/bonillakelvin/MetaSave/_git/Polkadot_FPS)

## Concept
Metasave is a data storage protocol for game worlds. It is a back-end service for game developers to manage player data in a decentralized, platform-agnostic way. The main goal of Metasave is to provide a mechanism where the consequences of one game carry over to any other game in real-time. Using permissionless read access, any game can react to events ocurring in any other game world irrespective of genre, platform, or hardware. To illustrate the concept, we will explore a hypothetical implementation of Metasave involving two popular games at the time of writing: 
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
![image](https://user-images.githubusercontent.com/1028926/141663167-cb933bdc-4e5d-4f0d-97e0-d3c83cd99146.png)  
User data is leveraged in a similar manner as world data, except it pertains to each individual user. This opens up novel use cases, such as anti-cheat & verifiable game progress. Other extraordinary implementations may include IoT.

#### Vending Machine IoT Example
In the diagram below, we explore how to bring [Final Fantasy VII Remake](https://www.playstation.com/en-us/games/final-fantasy-vii-remake/)'s vending machines into real life. 
![image](https://user-images.githubusercontent.com/1028926/139705304-0e95736c-9843-4a55-ac29-c593a15854f2.png)
**Why not just tokenize GIL (like an ERC-20)?**  
While tokenization is a valid (and sometimes ideal) strategy, it needs to be carefully considered...  
Tokenization often leads to relinquishing control of asset utility, which can adversely affect game design & balance. By restricting the data to a player's save file, the game designer is able to retain the quality control of their game's experience (the token asset doesn't run rampant through DeFi protocols). Furthermore, tokens alone are not enough to provide bespoke verification. In the example above, it is not enough to be able to afford the item. As a player, you must also progress far enough into the game in order to qualify by defeating Bahamut. The only way to accomplish this in a tokenized world is to mint yet another token (like an achievement) to track the progress. Ultimately, we end up juggling many tokens for a single game, which may be costly to transfer to other wallets/accounts, and/or also introduce many game-explot vectors (or even facilitate an unintended Pay-To-Win model via trading achievement tokens).

# Tech Specs
## Demo Usage
 For a thorough walkthrough of usage, please refer to the [video tutorial](https://www.youtube.com/watch?v=oZd8Vu2ZqiQ) linked at the top of this readme.  
General steps are outlined as follows:
1) Connect to the node using [Polkadot.js](https://polkadot.js.org/apps/#/explorer).
1) Select the `metasave` module from the extrinsics panel.
1) Register a game/world.
1) Add an authority (Optional. The account registering the game is an authority by default).
1) Update world data record to introduce any data to be parsed by a game client (Optional. Games clients/servers can do this on runtime).

## Tech Stack
*  [Substrate Gaming SDK](https://github.com/SubstrateGaming), already included within the game templates (see Deployment below).
*  [Unity3D 2019.4.23](https://unity3d.com/unity/whats-new/2019.4.23)

## Deployment
1)  Follow typical [node template](https://github.com/substrate-developer-hub/substrate-node-template/tree/v0.9.40) deployment (Metasave pallet is already included and installed). For this build, the node requires to be run in dev mode (`--dev` flag), so default accounts (Alice/Bob/Charlie/Dave/Eve/Ferdie) are accessible.  
1) Clone game templates.
    * [Polkadot_Platformer](https://dev.azure.com/bonillakelvin/MetaSave/_git/Polkadot_Platformer?version=GBweb3-hackfest)
    * [Polkadot_FPS](https://dev.azure.com/bonillakelvin/MetaSave/_git/Polkadot_FPS?version=GBweb3-hackfest)
1)  Open either game demo with Unity and play.

The Substrate node & game templates are preconfigured for local networking. You can redirect either game template's endpoint to the hosted demo node as well. Changing endpoints requires modifications only on the game clients.

## Notes
*  If you are unable to connect to the node via Polkadot.js, you are probably getting an invalid certificate error. Please follow the [official Polkadot instructions](https://wiki.polkadot.network/docs/maintain-wss#importing-the-certificate) to resolve this.
*  Metasave is preconfigured to have default Alice & Bob accounts registered with their respective games (see video for Alice & Bob setup). You may find the preconfiguration in the pallet's [lib.rs](https://github.com/kilogold/Metasave/blob/58ea78ee58966c82800c13d34c44e67bfc705c3e/pallets/metasave/src/lib.rs#L142).

