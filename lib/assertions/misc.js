const expect = require('chai').expect;

const assertIsPublicKey = (s) => {
    // '0x' followed by 33 hex-encoded bytes
    expect(s.match(/^0x[0-9a-f]{66}$/)).to.not.be.null;
};

const assertIsNull = (s) => {
    // '0x' followed by 33 hex-encoded bytes
    expect(s).to.be.null;
};

module.exports = {
  assertIsPublicKey,
  assertIsNull,
};
