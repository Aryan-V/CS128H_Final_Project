# How to use
1. Make sure your environment is set up correctly to build rust projects (you must have rust/cargo installed properly)
2. Clone repository with `git clone https://github.com/Aryan-V/CS128H_Final_Project` on the command line
3. Enter the directory (`cd CS128H_Final_Project`)
4. Retrieve an API key from https://newsapi.org/ and create .env file in the current directory with the environment variable NEWS_API_KEY. Set NEWS_API_KEY to the API key (i.e., NEWS_API_KEY=*key*).
5. You may also need to download the latest version of VS for the packages in this project to work.
6. `cargo run` (may encounter trouble when running on M1 MacBooks)
7. Wait for the project to run (it may take a while), and it should display a page window containing classified headlines
