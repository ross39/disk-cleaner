# A lightweight disk cleaner
Idea is that you could run a cron job/ scheduled task say once a week that will run this tool and clean up your pc/mac/linux machine. As dev's we download a lot of stuff, most of it to be never used again so it helps to keep your system clean. Besides its FOSS 
## How to determine junk files 
use regex for matching known junk file formats 

## List of junk files it will delete by default
.dmg
anything in the bin/trash
*.nav / Backup Files
*.nu3 / Symantec Backup File
*.old / Backup Files
*.prv / Backup Files
*.sdi / Archive Content File
*.sik / Potential junk
*.spc / Temporary Files (WordPerfect for Windows)
*.syd / Backup Files
*.temp / Temporary Files
*.tmp / Temporary Files
*.wbk / Word Backup Files
*.~* / Temporary Files
*__ofidx*.* / Microsoft Find Fast Indexer File
*ffastun / Microsoft Find Fast Indexer File
*.license.txt / Potential junk
*install*.txt / Potential junk
*.log.txt / Log Files
*modemlog.txt / Windows Modem Log File
*order*.txt / Potential junk

*.000
*.001
*.002
*.1st
*.b~k
*.bk
*.bmk
*.cam
*.cb
*.cln
*.cnt
*.da1
*.da2
*.diz
*.doc
*.edb
*.err
*.fix
*.ign
*.ink
*.lgc
*.lge
*.lic
*.new
*.out
*.par
*.pvt
*.query
*.sav
*.syd
*.umb
*.~mp
*.*_
*.!!!
*.lhx


## Can I change the list of files I want to delete? 
Yes you can edit the list to add/remove file extensions you wish to save or have deleted.

## Can this clear cache across my applications 
Yes I am working on a way to do this. 

## Disclaimer 
This comes with no guarantees of anything 

## Usage
disk_clean -d => This will run a default clean across the entire system
disk_clean -c => This will run a custom clean across the entire system
disk_clean -d /directory => this will run a default clean across a directory
disk_clean -c /directory => this will run a custom clean acorss a directory
