# TODO

- [x] Fix take-profit calculation to subtract (spread_pips + half_pip) for M trades to achieve true 1:1 R:R
  - Currently only adds `half_pip` back
  - Should subtract `(spread_pips + half_pip)` to mirror stop-loss adjustment
