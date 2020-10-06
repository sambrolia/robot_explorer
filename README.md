# Task
Rust library for calculating total possible area a robot can move on a 2d plane.  
- Coordinates shown as (x,y), where x and y are integers and movements are one space at a time to right, left, top, or bottom. Note: diaganols are not allowed.   
- Obsticles calulated by the sum of coordinate digits being less than or equal to a certain value -> Defaulted to 23.  

# Structure
This project has two crates within it.  
One, robot, is a lib crate which implements the functionality of the task as a usable library.  
The other, robot-explorer, is a bin crate which is a lightweight wrapper of the robot lib which lets you run the program.

# Improvements
Pass obsticle value in to the library as an argument, so that it can be changed based on usage to make the code more flexible. See: the way starting coordinates are implemented.  

When the obsticle value is set to much higher numbers, performance starts to become an issue so I would make two initial changes around this:  

1. Use parralisation around the location Queue so that we can take advantage of multi-core systems. I would organise this initially as batches, with each thread taking multiple values from the queue, and returning all the subsequent generated locations before grabbing another batch.  

2. Maths based improvements - when we calculate the possible routes, we do that calcuation for every single coordinate. This isn't needed as each movement can only add at most 1 to the digit sum when:  
    - Positive and increasing. 
    - Negitive and decreasing.  

This means that we could draw a semi circle around our current location, radius being equal to (object value - coordinate sum). We can add all the inner locations of the circle directly to our map of travelled locations, and add the outside locations in our circle to our stack to be analysed. Every time we move 10 elements in one direction, we gain a clear space ahead of 9 because of the base 10 number system. (50 = 5, 59 = 14, 60 = 6). This would make the group adding of locations by a drawn shape, regular, and effective.