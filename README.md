# CS128H Final Project

## Group Name:
ARE

## Group member names and NetIDs:
Aryan Vaswani (aryangv2), Rohan Bokaria (bokaria2), Edwin Ing (edwinji2)
## Project Introduction:
News article sentiment analyzer that allows a user to sort from a set of articles pulled from an API by emotion rather than topic. The analyzer can also assess the sentiments behind a given article based on the headline a user inputs.
## System Overview
- API for gathering news articles
- NLP algorithm on article text to gauge emotion [sentiment analysis]
  - Zero-Shot Classification Model
- Section for users to paste their own headlines and receive feedback from the model
## Possible Challenges
- Finding a labelled dataset with a comprehensive set of emotions
- Implementing an unsupervised model that updates with usage if building a recommendation system
## References
- https://towardsdatascience.com/multi-class-sentiment-analysis-using-bert-86657a2af156
