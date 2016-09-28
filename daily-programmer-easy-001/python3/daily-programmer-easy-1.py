#!/usr/bin/env python3

import time

name = input('What is your name, Oh Earthling?\n')
age = input('Fantastic! And how many times have you orbited about the sun?\n')
username = input('Excellent! Lastly, how do other humanoids refer to you on the information highway?\n')

result = 'your name is {0}, you are {1} years old, and your username is {2}'.format(name, age, username) 
print(result)

filename = 'python-log-' + str(time.time()) + '.log'
f = open(filename, 'w')
f.write(result)
f.close()
