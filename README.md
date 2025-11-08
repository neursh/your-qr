# Kewar
<sup>Yes, it's QR, but written as how it sounds</sup>

i'm just so sick of the current qr generating methods for non-tech people out there, all of the the qr code generating sites goes through themselves first to a trash, full of ads (rhyme intended) sits in between yea why not let's waste these people time and data.

## Privacy concerns
Kewar is built with privacy-focused in mind.

You don't need an account to start generating stateless QR code, the site will use your in-browser local storage to store your inputs, and it will renders on the fly when you need it.

There will be many features that will requires an account to access due to the nature of it (syncing for example). But you can absolutely use some of it without an account, but it will be active only for a short time.

Your account password will be hashed using scrypt (argon be really heavy ToT), and the data is going to be encrypted using AES-256.

For stateless QR, data stored on the server will be **forcefully** encrypted, and you will need to setup a master password to unlock it, the master password will stay on your browser. You might say it's kinda unnecessary because the QR will not be encrypted, why bother? Well idk, maybe the QR is used on-site, and it's not supposed to be known other than that.

For stateful QR, you can set a custom password to encrypt on the server, the but the people scanning it must know the password to unlock it, so please be unique for each stateful QR.

For some stateful QR that uses Kewar specific features, you have 2 options:
- The data WILL NOT BE encrypted, and the server can use it to update.
- The data WILL BE encrypted, and you must have an active tab running Kewar so the server can ask the client to update the data.

The second option might be a massive hassle (for me to make ;3), but it will provide a line of defense for data-sensitive stuff.

## API
You can grab yourself an API key on Kewar server to create stuff, and do stuff with it!

You can even get the server release files, setup and run your own Kewar server to your needs!

The normal Kewar user will be restricted from using any kind of QR generation on the server, API keys will have access to QR generation. The user won't need to abuse the server to create QR, they can do it themselves u know.

## Modularity
This is not a strict ecosystem.

The Kewar web doesn't have to be with the Kewar server, the Kewar server doesn't have to be with the Kewar web.

You can split Kewar web off, running it without server and it will work just fine, but without access to account related-features.

You can split Kewar server off, running it without web client and it will allow you to make other stuff related to QR over REST API.

## Plans
If this project managed to raised enough donations to not lose my lunch money running this thing, I'll consider getting a Pi 5 to run the server.

Since the Pi is not capable of handling much stuff, it would still goes through Cloudflare cache to ensure that it won't get hit as many. If the donations is stable enough, I'll change the host onto a VPS provider.

I'll work on server's foundation first, and then map out ideas to build for client.
