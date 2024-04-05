const express = require('express');
const app = express();
const port = 3000;
var bodyParser = require('body-parser')

const pokeElement = {
    name: "",
    url: ""
}

let pokemon = {
    Number: 0,
    Name: ""
}

app.use(express.json({limit: '50mb'}));
app.use(bodyParser.urlencoded({ extended: true, limit: '50000kb', parameterLimit: 50000 }))
app.use(bodyParser.json())



app.get('/', (req, res) =>{
    res.send('Gotta Catch Em All');
});

app.post('/pokecount', (req, res) => {
    
    let pokeElementList = []
    for(i = 0; i < 100_000; i++)
    {
        let count = 1;
        req.body.forEach(element => {
        pokeElementList.push(count, element.name);
        count++;
    });
    }

    res.send('gotta catch em all on: ' + pokeElementList.length);
});

app.listen(port, ()=>{
    console.log(`Example app listening on port ${port}`);
});