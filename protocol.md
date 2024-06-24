# Protocol

This is a TCP protocol for a version of tic-tac-toe.

Default port is `7777`.

- **C**: Player name, ASCII string terminated by `0x00`. Send only termination byte for no name.
- **S**:
  - `0x01` followed by opponent player name if match was successfully started.
  - `0x11` if the server game capacity has been reached. The server then expects the client to close the connection.
- Game starts

Until the game ends:

- **S**:
  - `0x01` followed by opponent move, if you are starting this is a null move.
  - `0x10` if connection was lost with opponent. The server then expects the client to close the connection.
  - `0x20` if the clients last move was illegal. A null move from the client counts as illegal.
- **C**: Player move.

When the game ends:

- **S**:
  - `0x02` followed by a move if the game ended with a three-in-a-row. If the move is a null move the client won. Otherwise the opponent won with that move.
  - `0x03` followed by a move if the game ended in a draw. If the move is a null move the client drew the game, otherwise the opponent drew the game with that move.
- **C**: Closes the connection.

## Move format

Coordinates are 2 2-bit numbers right after each other like this: `0bXXYY`, the game board goes from `0b0101` to `0b1111`, if either X or Y is zero the move is considered a null move.

## Errors

If either the client or server breaks the protocol and the other part detects the error it should immediately close the connection. Other communication errors sohuld already be covered by the protocol.
