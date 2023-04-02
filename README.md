## Inspiration
I am on my high school's esports team and I often have to analyze drafts because that is one of the most important parts about a game. It determines the dynamics of a game, the play styles, the lane states, and so much more. I looked around for any good websites that could analyze drafts and tell you what the best champions to pick are but they all were very rudimentary in their analysis. So I decided to build an AI to automatically do this.

## What it does
This AI will take in a current state and give you back what the best next possible picks are along with value ratings for those picks. It does this by analyzing every possible "game state" in the decision tree using a mini-max algorithm of elo-based game state heuristics. Now as you could imagine this is a very computationally challenging process which is why I had to implement several optimizations including:
- Alpha Beta pruning
- Transposition tables
- Iterative deepening
- Action ordering
- Bit wise hacking
- Optimized heuristic calculations
 and much more 

## How we built it
I utilized Rust as the back-end for all of the analysis and deep introspection into the game state, and a python flask server as an intermediary between the client and the rust back-end. All of the code was done from scratch including the mini-max algorithm and all optimizations creating a lot of overhead for this project.

## Challenges we ran into
I have only ever dabbled in rust before and this project was the equivalent of driving in a NASCAR after getting your permit, I was not prepared. Rust is a very different language to python (which is my bread and butter) so I was thrown for a spin. Through out the whole process of developing this I had issues and bugs all over the place and I was barely able to finish this project in time throwing out some extra sugar that I wished I could have kept in.

## Accomplishments that we're proud of
I am proud that I was able to complete this, and I am proud that I actually now understand Rust as a language. Before I had always felt too intimidated by it but now I know I can accomplish a lot with it and it is a very fun language to use. I am also proud of the progress in knowledge I have in Mini-max algorithms, AI, Elo, and much more that this project forced me to learn. 

## What's next for Draft-O-Matic
I hope to turn Draft-O-Matic into a full fledged desktop application with much more features and use cases as well as a much more optimized set of heuristics in the future with an optimized mini-max AI, it is definitely a useful project that will have a big impact on at least my esports team.

## How to run Draft-O-Matic

First you will need to run the `pull_data.py` then `parse_data.py` and then the `main.py` but this also runs a rust backend for doing the actual draft calculations so make sure that is installed.
