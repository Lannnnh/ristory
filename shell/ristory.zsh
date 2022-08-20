autoload -U add-zsh-hook

_ristory() {
	emulate -L zsh
	zle -I

	echoti rmkx
	output=$(ristory 3>&1 1>&2 2>&3)
	echoti smkx

	if [[ -n $output ]] ; then
		LBUFFER=$output
	fi

	zle reset-prompt
}

zle -N _ristory_widget _ristory

if [[ -z $RISTORY_NOBIND ]]; then
	bindkey '^h' _ristory_widget
fi
