var addon = require('../native');

var guard = new addon.AssetGuard('/test', '/val');

// Input any test files into this array. Full paths only.
console.log(guard.extractXZ([

]));
