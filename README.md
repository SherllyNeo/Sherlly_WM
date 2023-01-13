# Sherlly Windows Manager

similar to the style of dwm. Written in rust using penrose. 

How to use: 
Make sure you have X11 stuff installed.

Clone the repo, and you will get a binary called swm 

in your .xinitrc write
exec /path/to/swm/binary & 
and after you can put stuff to set the background and transparent terminals

here is an example .xinitrc

exec swm &
sleep 1
feh --bg-scale ~/backgrounds/wallpaper.jpg &
xcompmgr &
sleep infinity


after that
run startx to run the windows manager and profit!!

alternatively, use my autoinstaller scripts in jars on arch or the debian version to have this be the default window manager
