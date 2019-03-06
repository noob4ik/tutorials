# Fluence tutorials
This repository contains a list of different tutorials to play around with Fluence and learn how it works.

## [Database: llamadb](llamadb/)
[http://llamadb.fluence.network/](http://llamadb.fluence.network/)

A simple general purpose SQL database. You can build a lot of different DApps on top of it. Give it a try!

## [Game: tic-tac-toe](tic-tac-toe/)
[http://tictactoe.fluence.network/](http://tictactoe.fluence.network/)

Simple tic-tac-toe game showing how to make an interactive multi-user DApp use circular buffers to limit memory consumption. 

Take a look at the code, and try to come up with your own game! 

P.S. Maybe chess? :) There are some chess engines written in Rust. Look for most simple and single threaded implementations.

## [Game: guess a number](guessing-game/)
[http://guess.fluence.network/](http://guess.fluence.network/)

Inpsired by an example from [the Rust Book](https://doc.rust-lang.org/1.30.0/book/second-edition/ch02-00-guessing-game-tutorial.html). 

Following the code, it should be easy to build your own random-based privacy-free decentralized game backend.

## [Game: dice](dice-game/)
[http://dice.fluence.network/](http://dice.fluence.network/)

A simple dice roll game, where you can bet on the outcame, and earn or lose some points.

## [Integration: Pull analytics data from Streamr](streamr/)
This package is an example of how Fluence could be used to calculate different aggregates over analytics data in a decentralized fashion. Data is pulled from [Streamr](https://www.streamr.com/), stored in-memory with Llamadb SQL database, and then queried via Fluence JS SDK.
