Commands:
  Eco:
    Slots X - Spend X coins to spin the a 3 wheel slot, jackpot is 100x multipler of X
    Wheel X Y Z - Spend X coins to spin Y number of wheels with Z being experimental, jackpot is (X*100)^Y

  Store:
    Store - see the current store
    buy X - Buy item X with a single quantity count from the store
    buy X Y - buy item X with the quantity count of Y, some items prevent this
    sell X - sell item X with a single quantity count to the store, some items prevent this
    sell X Y - sell item X with the quantity count of Y, some items prevent this

  Person:
    mystats|me|mystats - gets your current stats
    inspect @ - get the stats of the @ player
    leaderboard - shows the overall leaderboard starting with highest overall
    leaderboard X - shows the leaderboard for X, can be str|coin|pvp (for pvp wins)|level
    inventory - see your current held inventory
    chest - see your stoarge chest
    bank X - put X in your storage chest
    cards - see your collectible card collection for cards you own
    card info X - see the card X in more details such as how to get, if&&when you obtained, and flavor text
    Zoltar - player pays 5 coins and Zoltar tells the player a flavor text regarding their fortune. This would tell the player which bonuses they secretly have activity for the day.

  Quest:
    quest - go on a quest and gain rewards equal to your level along with progressing in the current quest, cd 1d
    daily - go on a daily quest and gain rewards just below your current level, cd 1d
    dungeon X - attempt to go into a dungeon at X level, cd 10m
    
  Pet:
    pet - tells the player their current pet 
    petting-zoo - lists the pets the player owns
    change pet X Y - Change the pet X for the pet Y

  Exchange:
    give X @ - the current player requests to gives X to the @ player, 1hr cd
    take Y @ - the current player requests to take Y from the @ player, 1hr cd
    requests - the current player lists the active requests they have open
    approve # - the current player approves the request #

  Combat:
    pve fight - attack a monster at your present level
    pve fight 1 - attack a monster that is level 1
    pvp fight @ - request to fight @ player, 1hr cd

  Settings:
    delete - requests that the bot delete the entire player profile after 30 days of inactivity from the player
    privacy inspect - toggles ON/OFF if other players can inspect and see your character and makes you invisible in leaderboard
    exchange - toggles ON/OFF if other players can give or take from your character
