# Metasave
This project was developed for the Encode Polkadot Hackathon. 

## Concept
Metasave is a data storage protocol for game worlds. The main goal of Metasave is to provide a mechanism where the consequences of one game carry over to any other game in real-time. Using permissionless read access, any game can react to events ocurring in any other game world irrespective of genre, platform, or hardware. To illustrate the concept, we will explore a hypothetical implementation of Metasave involving two popular games at the time of writing: 
* Monster Hunter World
* Rainbow Six Siege

![image](https://user-images.githubusercontent.com/1028926/138626583-67dbe1a2-6991-43bc-bfd9-f45712ea6b8e.png)

## Architecture
### World Data Map (StorageDoubleMap)
![image](https://user-images.githubusercontent.com/1028926/138798123-5ba5866e-e222-4d33-a4a8-facd31159213.png)  
Keys and Values can be stored as any arbitrary data by virtue of a byte vector. This allows developers to choose and optimize world data to their desired use case. We could use a simple character string as a key, or we could use any arbitrary complex object represented in binary format.
### Authority Map (StorageMap)
![image](https://user-images.githubusercontent.com/1028926/138798404-5e994e26-8d95-4a24-a150-d7ea4717105c.png)  
An account is tied to a Permission which is evaluated every time storage data is modified. Game world data can only be modified by an authority (an account with permissions for said game). Each world data is partitioned into two categories:
* External
* Internal
