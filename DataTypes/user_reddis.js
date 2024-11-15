#!/usr/bin/node

const fs = require('node:fs/promises');

async function Rust() {
try {
    const rust =  await fs.readFile('/home/macron/100_days-of-Rust/DataTypes/user_reddis.rs',{ encoding:'utf-8'})
        console.log(rust);
    } catch (err) {
        console.error(err);
}
}

Rust();
