let rusty = require("rust-npm");

module.exports = async function (context, req) {
    context.log('JavaScript HTTP trigger function processed a request.');

    let matrixOne = JSON.stringify(req.body.matrixOne);
    let matrixTwo = JSON.stringify(req.body.matrixTwo);
    let result = rusty.multiply(matrixOne,matrixTwo);
    
    context.res = {
        body: {result: result}
    };
};