# Reasymotion
Reasymotion is an implementation of some basic easymotion-like functionality for Kakoune.

# Install
```
git clone https://github.com/astaugaard/reasymotion.git
cd reasymotion
cargo install --path .
```

Then add the following to your kakrc or load the file included in rc/reasymotion_kak
```
evaluate-commands $sh {
    rkak_easymotion start
}
```

# Commands
**reasymotion-selection**: runs reasymotion to keep one of your current selections 

**reasymotion-on-letter-to-letter**: allows you to input a letter then it will start reasymotion to try to jump to an instance of that letter

**reasymotion-on-letter-to-word**: the same thing as reasymotion-on-letter-to-letter, but instead only selects letters at the start of a word

**reasymotion-select-screen** \<keystrokes\>: select the whole screen then runs keystrokes and then runs reasymotion-selection (this is used to implement most of higher level functions)

**reasymotion-word**: use reasymotion to jump to a word on the screen

# Configuration

**reasymotion_keys**: keys to use when jumping to a selection.

default value: "abcdefghijklmnopqrstuvwxyz"



**REasymotionBackground** - the face that is used on the entire screen running reasymotion (currently not entirely working maybe?)



**REasymotionForeground** - the face that is used to display the "jump prompts"
