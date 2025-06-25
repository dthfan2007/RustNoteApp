# Dailies

## Day 1: 02.06.2025

On the first day, I first ran my idea by Reto really quickly, just to confirm wether I could do a project like this. After all, I didn't want to plan my project for the next few hours just for it to get turned down by him. After his confirmation, I began planning my project, utilizing GitHub Projects, just like I've already done for my other project that I realized while in the ZLI. Shortly after the lunch break, I presented my idea in a more detailed manner - every milestone and a few distinct goals that I wanted to reach. After getting the project signed off by Reto, I was able to begin my research on what I would actually need / use for this app. Very quickly, I found out about `egui` and `eframe`, which are 2 incredibly well documented crates that make it manageable to create a GUI for your app. As for safety, I chose to go with `argon2` for the password verification, while I decided to try out `chacha20poly1305` combined with `ring` or `orion` for the note encryption itself. I actually already got to test a bit with egui, where I tried to copy a basic application with name and age, that shows you how `egui` works and what is to expect when working with it.

## Day 2: 03.06.2025

Day 2 was mainly focussed on the documentation, as I knew from my last project, that it would get incredibly difficult to write a good documentation just during the last week, as you forgot a lot of things already. I didn't want to create a documentation with Word, as I had quite a few problems with it the last time I tried it, so I did some research into Markdown-Documentations enhanced with LaTeX and found out, that it is actually a viable alternative to create your documentation with. While the installation of all the things I needed (or I needed to update) took quite some time, I think that I'll get that time back by not having to wrestle with the layout on each page every time I try to insert a picture. In the afternoon, I first began by describing the project's guidelines, my project description and the risks I knew about before beginning my project. I then added the list of sources that I have already used, which there were more of than I first thought.

## Day 3: 04.06.2025

On day 3, I started implementing on implementing my basic features. I started by first designing a light-weight design for my app, with a simple GUI layout, so that the app wouldn't be hard to use. While I had setup the GUI fairly quickly, the saving / encryption process wouldn't be that fast. It took me way longer than expected to combine all my security features with each other, so that'd it actually be encrypted the way I described it in my project setup. But - after some trial and error, lots of documentation read and some help by v0, I got it to work. It now stores all data in the user's configuration directory. For windows, this would be `%APPDATA%\secure_notes\`, where it creates 3 files:

- `auth.hash`, which stores the password hash for authentication
- \sout{safety.meta} `security.meta`, which contains security metadata (hardware fingerprints, timestamps)

> Changed `safety.meta` to `security.meta` for more accurate file names

- `notes.enc`, which is the encrypted note data itself

By implementing an encryption key which is bound to hardware characteristics like username, operating system, computer name, etc..., this creates a hardware fingerprint that makes the encrypted data only accessible on the same machine that it was encrypted on.

## Day 4: 10.06.2025

The fourth day was also mainly focussed on documenting. I documented a few of my security features and how they work, and created the first entries into the glossary. I also created a Gantt diagram that shows how I initially planned my project to develop, to which I will add a Gantt of how it actually developed over the days once the project is finished. The app itself did not change much this day, as I had some catching up to do with my documentation. I only changed the UI a bit and also decreased the security of the app a little bit - to a point where it still is theoretically safe for production, but the user doesn't have to wait 26 seconds every time they want to log in.

## Day 5: 11.06.2025

The fifth day was overshadowed by a simple, yet embarrassing error that I made. When I was re-reading my code that I wrote over the past week, I noticed that I set the filename for security metadata to `safety.meta`, which I decided to change to `security.meta`, as that was a more fitting name for me. I also noticed that I was using `allocate_ui_at_rect()` to display my notes in the statusbar on the left, which I tried to refactor to `allocate_new_ui()` as `allocate_ui_at_rect()` is now deprecated and been replaced by `allocate_new_ui()`. When I ran the project for the first time again, the login just didn't work anymore. I got an error that my password was not matching, even though I entered the correct password. It took me an embarrassing amount of time to realize that it wasn't the new `allocate_new_ui()` approach that I took that was causing the error - which wouldn't have made any sense anyways since that only gets loaded once the login has already been completed - but the name change from `safety.meta` to `security.meta` that caused the error during login, as it searched for a file that didn't even exist. This is what caused the authentication error. I thusly had to delete all the files that have been created in `%APPDATA%\secure_notes\`, so that the program would think that it's a first time setup again, so I can create a new master password.

## Day 6: 16.06.2025

Day 6 was shaped by some more documenting. Early in the morning, Reto reminded us that each of our repositories needed a README file, which I didn't yet create. So I had to work on that before I could do anything else. It took me a bit longer than expected because I wanted to create a clean and easy-to-read README file, that can be understood by anyone. I also added some parts where I'm not sure if I want to keep them in over longer periods of time, as they could grow to be untrue / not implemented.

## Day 7: 17.06.2025

Day 7 was - unfortunately - quite an unproductive day. I tried my best at implementing user logins and getting the login to persist for _x_ amount of time. However, I had some problems with storing all the user data and getting the right one to decrypt their password, and with showing them the right notes, that I had to call that mission off quite quickly. I then looked back at my documentation, and added some more words to my glossary. I then also went over my GitHub Roadmap again, for which I marked off the issues that I have now completed.

## Day 8: 18.06.2025

On the eighth day I was working from home. I was able to implement User Login / Registration after dealing with it for some time. Then I had to fix a problem where _all_ notes would be shown to every user once they've logged in. I fixed it by modifying how notes are stored. Now every user has their own folder with the 3 files in it. This means that the size of the storage goes up slightly, but it was the only way that came to mind fairly quickly. Deleting a note is now also stored in a context menu that pops up on right click, and not just always available as a button.

## Day 9: 20.06.2025

Day 9 was an additional day that I got to work on my project on, as school got cancelled. I redesigned a few things in the app, although you can't really see that much of it because it is mainly just minor adjustments in the layout. I want to add a few different color schemes at some point, but I don't know how hard that might be to implement. I also split my code even further, as my `main.rs` file got pretty long. Now I use 9 files instead of the old 3. Weirdly enough, the decision on _how_ to split the files itself was more complicated than the splitting of the files itself.

## Day 10: 23.06.2025

Day 10 was another day that was mostly spent on the documentation. I read through my `crypto.rs` file again, as that contains a fair amount of code that I just don't understand fully. After reading through the entire file and trying to understand all of the code by various means, I tried my best at documenting the whole process more in-depth than it has been so far. For the remainder of the afternoon, I documented my most important functions and added their code snippets to my appendix. This process took quite some time, as I had to look at every function again to decide which ones I want to document and which ones I will leave out.

## Day 11: 24.06.2025

Day 11 was a really productive day, as the feeling of "I don't know if I'm really going to be able to finish everything" has started to kick in. I worked on implementing Keyboard Shortcuts, which took surprisingly little time. After that, I worked on exporting the files to `.txt` format, which turned out to be a little more annoying than I expected, not because it was difficult to implement, but because it just didn't work for 30 minutes, and then suddenly - without changing _anything_, it just worked. In the afternoon I updated my glossary and added terms, which I have used in this documentation and I feel like they could need some further explanation.
