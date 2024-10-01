var addon = require('../native');

// console.log(addon.hello());
module.exports = {
    hello: addon.hello,
    parseAsync: addon.parseAsync,
    listenMouse:addon.listenMouse
};
