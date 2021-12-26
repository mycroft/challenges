# Codes

## To do / to implement

* Implement basic op codes;
* Implement all op codes;

## First code

It is actually in the `arch-spec` file: `2,Hjq:1N+gt:Bmb_`

## Second code

After having implemented basic op codes (0, 21 & 19), first code appears:

```
Welcome to the Synacor Challenge!
Please record your progress by putting codes like
this one into the challenge website: RQKepPzSogLV
```

## Third code

After implemented all op codes, the self-test will complete and will show:

```
self-test complete, all tests pass
The self-test completion code is: DxmwWHxLwSMl
```

# Fourth code

When playing:

```
take tablet
use tablet

You find yourself writing "sTKZwnyZATmF" on the tablet.  Perhaps it's some kind of code?
```

# Fifth code

Found by just playing

```
take tablet
doorway
north
north
bridge
continue
down
east
take empty lantern
west
west
passage
ladder
west
south
north
```

Will lead to:

```
Chiseled on the wall of one of the passageways, you see:

    mFgjKtcHWhAX

You take note of this and keep walking.
```

# Sixth code

Continue playing:

```
take can
use can
west
ladder
darkness
use lantern
continue
west
west
west
west
north
take red coin
north
west
take blue coin
up
take shiny coin
down
east
east
take concave coin
down
take corroded coin
up
west
```

And then you'll be in here:

```
== Ruins ==
You stand in the massive central hall of these ruins.  The walls are crumbling, and vegetation has clearly taken over.  Rooms are attached in all directions.  There is a strange monument in the center of the hall with circular slots and unusual symbols.  It reads:

_ + _ * _^2 + _^3 - _ = 399
```

Found values:

* red coin: 2
* corroded coin: 3
* shiny coin: 5
* concave coin: 7
* blue coin: 9

After a few seconds of math:

```
>>> 9 + 2 * 5**2 + 7**3 - 3
399
```

So the correct order is:

```
use blue coin
use red coin
use shiny coin
use concave coin
use corroded coin
```

And the result will be:

```
You place the corroded coin into the leftmost open slot.
As you place the last coin, you hear a click from the north door.
```

```
north
take teleporter
use teleporter
```

Will output the next code:

```
You activate the teleporter!  As you spiral through time and space, you think you see a pattern in the stars...

    AIwLjfSiytZR

After a few moments, you find yourself back on solid ground and a little disoriented.
```

# Seventh code

```
take business card
take strange book
look strange book
```

TODO.

# Eighth & latest code

TODO.
