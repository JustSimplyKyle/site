Ever played rhythm games?
I have, and I LOVE it.

One nasty thing though, is that it's often recommended that we attach a headphone when playing.

For people like me who have trouble wearing headphones for over 30 minutes, do we deserve to suffer listening to obviously inferior phone speakers?

# The first attempt
My first attempt to hooking up my phone to a speaker, is through a DAC. A Digtal to Analog Converter.

Through it, it's possible for a phone without a headphone jack to "have a headphone jack" again, with added benefits such as imporved audio quality.

I did end up getting it to work, but it's exteremly cumbersome.

I first have to DETACTH my speakers from my computer, then connecting the DAC to my phone, finally stringing them all together with a long cable that from time to time, will tangle you when you're doing your FC attempt.

"There must be a better way!" is what I'd thought.

# The second attempt
The gist of it is the same, somehow connecting my phone to a speaker, this time however preferably without a converter.

One program came to mind, `scrcpy`. It's primrary functionality is to share your screen to a computer, using a just a humble type c cable. 

My naive thinking process was, "since it can transmit video, how can it not also has the ability to transmit audio!".

I couldn't be more wrong. Not only it has no such functionality, but no good alternatives exists. 

The best candidate is `sndcpy`, which to their credit, works pretty good! Except for the fact that it has latency as high as 500ms, making it quite unsuit for rhythm game purposes.

# The Final attempt(working)
Time flies, and 3 years past. And you know what? `scrcpy` introuced their sound sharing functionality!

What's more, it's latency is acceptable, being around 200ms, which is the latency typical for bluetooth headphones.

So, for I to play rhythm games with a good speaker, all I have to do was this.
1. Connect my phone to my pc.
2. Run `scrcpy --no-video --audio-buffer=20` on my computer.
3. Set the latency of the rhythm game in question to be around `200ms`.
4. tada! I have `scrcpy` set up for rhythm game business.

# tl;dr
Sometimes, when you can't find a solution, wait for 3 years, and someone else somewhere on this globe will figure it out for you.
