const math = require('mathjs');

module.exports = async function (context, req) {
    context.log('JavaScript HTTP trigger function processed a request.');

    let matrixOne = math.matrix(req.body.matrixOne);
    let matrixTwo = math.matrix(req.body.matrixTwo);
    let result = math.multiply(matrixOne,matrixTwo);
    
    context.res = {
        body: {result: result._data}
    };
};