# Task
Rust library for calculating total possible area a robot can move on a 2d plane.  
- Coordinates shown as (x,y), where x and y are integers and movements are one space at a time to right, left, top, or bottom. Note: diaganols are not allowed.   
- Obsticles calulated by the sum of coordinate digits being less than or equal to a certain value -> Defaulted to 23.  

# Structure
This project has two crates within it.  
One, robot, is a lib crate which implements the functionality of the task as a usable library.  
The other, robot-explorer, is a bin crate which is a lightweight wrapper of the robot lib which lets you run the program.  

The first idea I had was simple recusion, but realising the limits of the call stack, I implemented my own queue to allow for much larger sets of data.  

Each location is popped off the queue, and then all safe adjacent locations are added to the visited map. If they did not already exist in the map, they are added to the queue to be processed in turn.    

# Improvements
Pass obsticle value in to the library as an argument, so that it can be changed based on usage to make the code more flexible. See: the way starting coordinates are implemented.  

When the obsticle value is set to much higher numbers, performance starts to become an issue so I would make two initial changes around this:  

1. Use parralisation around the location Queue so that we can take advantage of multi-core systems. I would organise this initially as batches, with each thread taking multiple values from the queue, and returning all the subsequent generated locations before grabbing another batch.  

    This would not effect the time complexity (it may actually add slight overhead), but would add throughput in taking advantage of the hardware more efficiantly.

2. Maths based improvements - when we calculate the possible routes, we do that calcuation for every single coordinate. This isn't needed as each movement can only add at most 1 to the digit sum when:  
    - Positive and increasing. 
    - Negitive and decreasing.  

    This means that we could draw a semi circle around our current location, radius being equal to (object value - coordinate sum). We can add all the inner locations of the circle directly to our map of travelled locations, and add the outside locations in our circle to our stack to be analysed. Every time we move 10 elements in one direction, we gain a clear space ahead of 9 because of the base 10 number system. (50 = 5, 59 = 14, -> , 60 = 6). This would make the group adding of locations by a drawn shape, regular, and effective.  

    I believe this would bring a time complexity from the O(n) of inspecting every coordinate, to an O(log n) as the larger the datasets and obsticle values, the larger the shapes which can be drawn and processed in one go.


3.   If space complexity became an issue, it may be possible to combine the map and queue into one datastructure with a status to say whether each element has been processed (the queue side), whilst maintaning the key-value funtionality of the map in terms of detecting duplicates and quick lookup times.  
    I Would rather not implement this unless strictly nessesary as I believe it would be more complex to understand, more complex to work on, and likely add time complexity considerations in terms of navigating a larger data structure.  The simplicity of a queue and a map is imediately understandable to any consumers of the code. It would amount to early optimisation.  