# Designing a SVG slider

It's time to try our hand at designing a reusable encapsulated component.
I've chosen an audio slider, the likes of which you see on mixing desks and in fancy DAW software, as it is visually interesting and illustrates key concepts of reusable components!

In this chapter, we will implement our slider using vanilla **dominator**.
We will revisit this component once we cover the dominator macros from DMAT in a later section.

Let's first come up with the properties we need to model in our component:

* It must allow us to control a numeric value linearly between a maximum and minimum value
* We want to allow the user to disable/enable the slider input
* It must look like its physical counterpart
* Ideally we want to allow the user to customize the look of our slider (within reason)

## Figuring out the data model

The first thing to do when designing any UI component, is to figure out what data it will operate on, and how we best represent it in our code.

For this slider, there are at least two identifiable data points we can start with.

The most important is the value of the slider.
This can be modelled as a simple number, representing the point along the linear axis of the slider the knob is currently at.

We can model this as a simple `Mutable<f64>`, with a user configured value range.
There are problems with this choice if we want to make our component library as widely adoptable as possible though.

First of all, choosing a concrete number type forces the user to adopt his data model to our chosen representation.
What if he has a `u64` value? 
What if he doesn't want to store the value in local state, but rather forward control messages to a physical control board with a motorized slider?

Both the choice of value type and container is problematic when designing reusable code.

We will now first implement the naive `Mutable<f64>` solution, and then we will take the effort of generalizing it so that we can analyze the difference.

## Mocking the component signature

Now that we have settled on our initial data model, let's sketch out a function signature for our slider:


```rust,no_run,noplayground
{{#include ../doc-imports/src/design_essays/audio_slider.rs:audio_slider_sketch}}
```

[todo: insert profound tutorial text here]

Finally, our naive implementation of the audio slider looks like this:

```rust,no_run,noplayground
{{#include ../doc-imports/src/design_essays/audio_slider.rs:audio_slider_final}}
```