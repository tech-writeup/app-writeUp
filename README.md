## writeUp!

writeUp is a Electron based decentralized note-taking application for the web and desktop, written in Rust/JS.

writeUp uses IPFS to store your notes securely, offering you a seamless experience at 0 cost.

---

### Setup

#### Running the Server

1. `cd server`
2. `node index.js` - This will start the interim server

#### Running the client

_Debug_

1. Go to the root of the project
2. `npm run start` - Run the debug mode of the project

_Release_

1. `npm run build` - Build the client
2. `npm i -g serve` - Install the `serve` package executable
3. `serve -s build` - Run the production build

Inspired By & Forked to Solana from - https://github.com/jotterdapp/jotter
