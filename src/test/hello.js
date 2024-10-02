const rustModule = require('mouse_listener');

console.log(rustModule);
let x=rustModule.hello()
console.log(x)


rustModule.listenMouse((error,result)=>{
  console.log(result);
})
