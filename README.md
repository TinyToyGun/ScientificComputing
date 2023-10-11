# ScientificComputing
Repository for the programming exercises of the Course Scientific Computing of Luis Lukas and Paul

For Lukas and Paul.
It's best you install WSL (windows subsystem for Linux) to have a Linux environment to work in. In the Microsoft store (https://www.microsoft.com/store/productid/9P9TQF7MRM4R?ocid=pdpshare)
the open it, and run the command wsl --install. if it says it can't install Ubuntu, you might have to turn on virtualization in your BIOS settings. 
Search what's the best way to get into the BIOS settings and search for the setting that says something about virtualization and enable it. after you have your wsl running
make a directory (mkdir name_of_folder) for the project. 
Then on to installing rust, type this command "curl https://sh.rustup.rs -sSf | sh" (ctrl-c and ctrl-v dont work in the command line -.-) and follow the steps 
to install cargo and rust. 
Lastly, I already set up the first folder with Rust, and you can just pull the repo. for that, you will have to have a GitHub account. After you have set up
your GitHub account, click on your profile, and on the left all the way to the bottom there is "Developer Settings". Here you are gonna create a classical Personal Acces Token,
check all the boxes that there are to check, and set the expiry date to whenever (I set 29.02.2023). Copy the access Token it gives you and save it somewhere. Finally you 
can pull the GitHub repo. In wsl go to the folder you made for the project, and type "git clone https://github.com/TinyToyGun/ScientificComputing.git", it is then gonna ask for your 
username and password, but here you don't type your account password in, but the personal access token (i think it only needs to be done this once). After that, we are set to work 
together on some rusty code.
