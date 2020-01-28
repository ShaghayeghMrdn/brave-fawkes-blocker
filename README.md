# brave-fawkes-blocker
Using Brave Adblocker to analyze URLs requested in Fawkes page load

## Overview:
I have recorded reddit homepage load 8 hours apart using [mahimahi](http://mahimahi.mit.edu/). Then using [Fawkes](https://github.com/fawkes-nsdi20/fawkes) I have generated a version of the patch that serves the static template and dynamic patch as well as another version which only serves the static template.

Then I have logged the URLs that are requested in each of these page loads versus the default page load (replayed using mahimahi),.
These url files are then checked against the [brave adblocker](https://github.com/brave/adblock-rust) to see how many of them of are matched with the given adblocking rules.

## Setup:

- Clone the repository with all of its submodules:
    ```
    $ git clone --recursive git@github.com:ShaghayeghMrdn/brave-fawkes-blocker.git
    ```

- For tracking new commits:
    ```
    $git submodule update --remote
    ```

## Breakdown:
The following instructions demonstrates how to generate the results for reddit homepage. We can use similar commands to generate the results for yahoo homepage as well.
Two example directories (for two web pages) are given under *adblock-examples*.

### Input directories:
- *record-b2b* and *record-08h* subdirectories are examples of back to back and 8 hours apart page loads, recorded using mahimahi.
- *v0* and *v1* are the actual mahimahi directories recorded at initial time and target time (back to back or 8 hours apart) respectively.

### Fawkes vs default directories:
To generate the replay directories for reddit (as an example), run the following commands:
  ```
  $ cd fawkes
  $ ./fawkes-nopatch.sh $(pwd)/../adblock-examples/reddit/record-b2b
  $ ./fawkes-nopatch.sh $(pwd)/../adblock-examples/reddit/record-08h
  ```

The outputs are placed under *adblock-examples/reddit/replay-b2b*  *adblock-examples/reddit/replay-08h*.

Each of the the above directories contains three mahimahi directories which can be replayed: *default* (default page load), *fawkes* (our version including the static template and dynamic patch), *fawkes_nopatch* (our version only including the static template).

### List of requested URLs:
From the fawkes root directory, you can use replay.sh script to replay each of these directories on LTE Cellular network with 50ms delay:
  ```
  $ ./replay.sh ../adblock-examples/reddit/replay-08h/fawkes-nopatch/
  ```

Then run the following inside the replay shell:
```
$ cd extra-scripts/requested-urls/
$ ./all-urls.sh https://www.reddit.com ../../../filtered-urls/data/reddit-nopatch-urls.txt
exit #exits the mahimahi shell and goes back to Fawkes root directory
```

*all-urls.sh* loads the page using the provided chrome binary (without UI) and outputs a list of requested URLs on the network before the page load is completed. The output is placed under data directory.

### Running against Brave adblocker:
Now we can run brave adblocker against these lists using the following:
```
$ cd filtered-urls/
$ cargo run https://www.reddit.com data/reddit-default-urls.txt data/reddit-nopatch-urls.txt
```

It prints the number of matched urls in the default page load vs Fawkes static template.
For reddit example, **9 urls are matched in the default page load, whereas Fawkes static template did not contain any matched urls.**
