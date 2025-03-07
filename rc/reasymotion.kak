
decl -hidden str _scrolloff

decl -hidden range-specs reasymotionselections

decl -hidden str reasymotion_command

set-option global reasymotion_command "rkak_easymotion"

decl -hidden str reasymotion_last_selection 

decl str reasymotion_keys

set-option global reasymotion_keys "abcdefghijklmnopqrstuvwxyz"

face global REasymotionBackground rgb:aaaaaa
face global REasymotionForeground white,red+F

def reasymotion-select-screen -params 1 %{
    set-option window _scrolloff %opt{scrolloff}
    set-option window scrolloff 0,0

    execute-keys "gbGt%arg{1}<a-:>"

    reasymotion-selection

    hook window -once NormalKey .* %{
        set-option window scrolloff %opt{_scrolloff}
    }
    hook window -once NormalIdle .* %{
        set-option window scrolloff %opt{_scrolloff}
    }}

def reasymotion-select-screen-expand -params 1 %{
    set-option window _scrolloff %opt{scrolloff}
    set-option window scrolloff 0,0

    set-option window reasymotion_last_selection %val{selections_desc}

    execute-keys "gbGt%arg{1}<a-:>"

    reasymotion-selection-expand %opt{reasymotion_last_selection}

    hook window -once NormalKey .* %{
        set-option window scrolloff %opt{_scrolloff}
    }
    hook window -once NormalIdle .* %{
        set-option window scrolloff %opt{_scrolloff}
    }}

# gbGt to select whole screen
def reasymotion-selection %{

    add-highlighter buffer/reasymotionselections replace-ranges reasymotionselections
    add-highlighter buffer/reasymotionbackground fill REasymotionBackground


    evaluate-commands %sh{
        # need enviroment variables
        # (can't remove because otherwise kak doesn't export them so the program can't access them)
        # $kak_selections_desc $kak_opt_reasymotion_keys
        $kak_opt_reasymotion_command start
        # rkak_easymotion start
        }

}

def reasymotion-selection-expand -params 1 %{

    add-highlighter buffer/reasymotionselections replace-ranges reasymotionselections
    add-highlighter buffer/reasymotionbackground fill REasymotionBackground

    evaluate-commands %sh{
        # need enviroment variables
        # (can't remove because otherwise kak doesn't export them so the program can't access them)
        # $kak_selections_desc $kak_opt_reasymotion_keys

        export EXTEND_SELECTION=$1

        # rkak_easymotion start
        $kak_opt_reasymotion_command start
        }

}

def reasymotion-line %{
    reasymotion-select-screen <a-s>x
}

def reasymotion-line-expand %{
    reasymotion-select-screen-expand <a-s>x
}

def reasymotion-word %{
    reasymotion-select-screen s\w+<ret>
}

def reasymotion-word-expand %{
    reasymotion-select-screen-expand s\w+<ret>
}

def reasymotion-on-letter-to-word %{
    on-key %{
        reasymotion-select-screen "s\b%val{key}\w*<ret>"
    }
}

def reasymotion-on-letter-to-word-expand %{
    on-key %{
        reasymotion-select-screen-expand "s\b%val{key}\w*<ret>"
    }
}

def reasymotion-on-letter-to-letter %{
    on-key %{
        reasymotion-select-screen "s%val{key}<ret>"
    }
}

def reasymotion-on-letter-to-letter-expand %{
    on-key %{
        reasymotion-select-screen-expand "s%val{key}<ret>"
    }
}
