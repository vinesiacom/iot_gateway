const crypto = require('crypto');
const { Ed25519KeyIdentity } = require('@dfinity/identity');
const fs = require('fs');

const entropy = crypto.randomBytes(32);
const key = Ed25519KeyIdentity.generate(entropy);

fs.writeFileSync('key.json', JSON.stringify(key.toJSON()));