import random
import string

def randChar():
    return random.choice(string.printable)

def randString(length):
    return ''.join(random.choice(string.printable) for elem in range(length))

def hammingDistance(s1,s2):
        ham = 0
        if len(s1) != len(s2):
            return float("inf")
        for c1, c2 in zip(s1, s2):
                if c1 != c2:
                        ham+= 1
        return ham

def initialize(population):
    fittest = min(population, key=fitness)
    score = fitness(fittest)
    return (fittest, score)

def offspring(p1, p2):
    (f1, f2) = [fitness(x) for x in (p1, p2)]
    child = ''
    for c1, c2 in zip(p1, p2):
        if random.random() < rateOfMutation:
            child += randChar()
        else:
            child += random.choice(c1*f1 + c2*f2)
    return child

if __name__ == "__main__":
    targetString = input('Enter a target string: ')
    fitness = lambda stringEval : hammingDistance(stringEval, targetString) #returns equiv of a js closure
    rateOfMutation = 1 / len(targetString)

    generationSize = 100
    currentPopulation = set([randString(len(targetString)) for elem in range(generationSize)])
    currentGeneration= 1

    (fittest,score) = initialize(currentPopulation)
    while score != 0:
        (fittest,score) = initialize(currentPopulation)
        print(str(currentGeneration) + " | " + str(score) + " | " + fittest)

        parent1 = fittest
        parent2 = min((currentPopulation - set([parent1])), key=fitness)
        currentPopulation = set(offspring(parent1, parent2) for elem in range(generationSize))
        currentGeneration += 1
    print('Done')

