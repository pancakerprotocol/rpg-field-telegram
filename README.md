# rpg-field-telegram
Telegram bot for RPG field fantasy

This is mostly to act as a fun RPG stats bot tool.

Stages:
Learn Telegram API/bots
build hello world bot
build slots
build store
build user stats
build leaderboard (also leaderboard for each stat)
build daily quest
build PvE
build PvP
build mini pet collect
build mini pet arena
build collectible cards
build Tolzar
build NSFW

To Do:
  User:
    View user
    change user
    view inventory
    equip items
  Collectible cards:
    Description: cards that can be found while playing with the bot, can also find cards randomly or with jackpot
      #1:
        Use HP:
          use a health potion for the first time
      #2:
        Use MP:
          use a Mana potion for the first time
      #3:
        Use EP:
          use an Energy Potion
  Leaderboard
    Description: users can sort based upon the values
      Gold
      Monsters killed
      potions used:
  Tolzar:
    Tells players a riddle about their daily hidden random bonus:
      attack:
        deal additional 1d6 of damage
      slots:
        increase odds by 0.010%
      potions:
        add 5% to potions restore
  Slots:
    Description: Where players can gain coin over time:
    Use: 
      gain 1 coin if pouch is under 1m
      Jackpot odds: 0.001%
      If over 1m then negatives are possible
      if jackpot gain gold or random collectible card
  Shop:
    where players can buy items using coins
      sword:
        Basic:
          equip: deal 1d6 points of damage
          cost: 10 coins
        Advanced:
          equip: deal 2d6 points of damage
          cost: 20 coins
        Expert:
          equip: deal 3d6 points of damage
          cost: 30 coins
        Master:
          equip deal 4d6 points of damage
          cost: 40 coin
      staff:
        Basic:
          equip: deal 1d6 points of damage
          cost: 10 coins
        Advanced:
          equip: deal 2d6 points of damage
          cost: 20 coins
        Expert:
          equip: deal 3d6 points of damage
          cost: 30 coins
        Master:
          equip deal 4d6 points of damage
          cost: 40 coin
      bow:
        Basic:
          equip: deal 1d6 points of damage
          cost: 10 coins
        Advanced:
          equip: deal 2d6 points of damage
          cost: 20 coins
        Expert:
          equip: deal 3d6 points of damage
          cost: 30 coins
        Master:
          equip deal 4d6 points of damage
          cost: 40 coin
      Health potions:
        Basic:
          use: gain 5%-10% health
          cost: 10 coin
        Advanced
          use: gain 10%-15% health
          cost: 25 coin
        Expert:
          use: gain 15%-20% health
          cost: 50 coin
        Master:
          use: gain 20%-25% health
          cost: 75 coin
      Energy potion:
        Basic:
          use: gain 5%-10% health
          cost: 10 coin
        Advanced
          use: gain 10%-15% health
          cost: 25 coin
        Expert:
          use: gain 15%-20% health
          cost: 50 coin
        Master:
          use: gain 20%-25% health
          cost: 75 coin
      revive potion:
        Single:
          use: revive with 5-10% health
          cost: 1000 coins
        Double:
          use: revive with 10-15% health
          cost: 2000 coins
        Triple:
          use: revive with 15%-20% health
          cost: 3000 coins
        Quad:
          use: revive with 20%-25% health
          cost: 4000 coins
