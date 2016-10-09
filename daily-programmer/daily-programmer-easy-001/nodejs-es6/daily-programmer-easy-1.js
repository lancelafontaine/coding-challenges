// npm install -g babel-cli
// babel-node logAndOutputUserInfo.js

const readline = require('readline');
const fs = require('fs');

let name, age, username;

const rl = readline.createInterface({
  input: process.stdin,
  output: process.stdout
});

const askQuestion = (prompt, cb) => {
  rl.question(prompt, (answer) => cb(answer));
};

const run = (cb) => {
  askQuestion('What is your name?\n', (nameAnswer) => {
    name = nameAnswer;
    askQuestion('What is your age?\n', (ageAnswer) => {
      age = ageAnswer;
      askQuestion('What is your username\n', (usernameAnswer) => {
        username = usernameAnswer;
        rl.close();

        const output = `your name is ${name}, you are ${age} years old, and your username is ${username}`;
        console.log(output);
        cb(output);
      })
    });
  });
};

const writeToFile = (output) => {
  const filename = 'javascript-log-' + Math.floor(new Date() / 1000) + '.log';
  fs.writeFile(filename, output, (err) => {
    if (err) throw err;
  });
};

run(writeToFile);
