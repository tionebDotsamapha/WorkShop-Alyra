# WorkShop-Alyra

Le 18 oct 2023 je serais invité chez Alyra pour présenter PhalaNetwork aux devs.

Mon but sera de démontrer combien Phala est un putain de projet a même de changer le monde pour un Web3 respectueux et respectable.

<p align="center">
  <img  width="460" height="300" src="https://github.com/tionebDotsamapha/WorkShop-Alyra/assets/16654460/3d3505b9-7c0f-45ad-9d82-746335ef3413">
</p>

Ce workshop est build sur une machine Linux Debian. 
Basé sur une install Docker, ce workshop est reproductible sur Microsoft ou la pomme.

Il existe une communauté PhalaNetwork FR maintenue par des ambassadeurs officiels. 
Nous sommes plusieurs à pouvoir vous aider au lancement de vos projets sur le PhatContract.
Il vous suffit de rejoindre le discord :

👉 https://linktr.ee/phalafr,
 
👉 de pinguer @tioneb pour demander un call,

OU

👉 taguer les ambassadeurs de la team phalafr @phamb



## prérequis

Docker et Docker-compose doivent être insttallés.

Pour vérifier :

``` sudo docker --version && sudo docker-compose version ```

Vous devriez obtenir ceci : 

<img width="257" alt="image" src="https://github.com/tionebDotsamapha/WorkShop-Alyra/assets/16654460/f9fcb9b5-b60b-4402-8e5c-69daab67282c">

Si ce n'est pas le cas et afin d'éviter des conflits de version, votre serviteur vous à concocté un joli tuto disponoble sur le discord PhalaTeamFR:
https://discord.com/channels/949641126946693141/1035295622057689148/1035296203333697557
## Installation de l'environnement de develloppement.
*18 MINUTES*

### Installer git, Cloner l'installation minimal et lancer la construction du container.

``` 
sudo apt install git -y && \
git clone https://github.com/tionebDotsamapha/WorkShop-Alyra-1_basicInstall.git && \
cd WorkShop-Alyra/ && \
sudo docker-compose build && \
sudo docker-compose up -d
```
### Lancer le shell du container de dev
```
docker exec -ti ws_alyra /bin/bash
```
Après avoir lancé cette commande, vous ne travaillez plus sur votre machine hôte, mais à l'interieur de votre container docker.
Pour en sortir et revenir à votre machine hôte il suffit de tapper: 
```
exit
```
### Créer un nouveau projet.
```
cargo contract new ws_alyra && \
cd ws_alyra
```
A ce stade, vous beneficiez d'un environnement DEV de smartcontracts ink, tout neuf.
La création de ce nouveau projet implémente un SmartContract leger, de type hello-world mais avec une simple variable hello-world.

### tester et Compiler avec Rust.
- Pour tester le bon fonctionnement de ce smart-comtract, lancez :  
```
cargo test -- --nocapture
```
**cargo test :** lance l'execution des test
*10 MINUTES*

- Pour compiler et obtenit le smart-comtract à inscrire onChain, lancez :
```
cargo contract build
```
*2 MINUTES*

La suite dans le dossier : 1_Price_Oracle....
