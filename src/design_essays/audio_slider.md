# Designing a SVG slider

It's time to try our hand at designing a reusable encapsulated component.
I've chosen an audio slider, the likes of which you see on mixing desks and in fancy DAW software, as it is visually interesting and illustrates key concepts of reusable components!

In this chapter, we will implement our slider using vanilla **dominator**.
We will revisit this component once we cover the dominator macros from DMAT in a later section.

Let's first come up with the properties we need to model in our component:

* It must allow us to control a numeric value linearly between a maximum and minimum value
* We want to allow the user to disable/enable the slider input
* It must look like its physical counterpart

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

## Defining the component signature

Now that we have settled on our initial data model, let's sketch out a function signature for our slider:

```rust,no_run,noplayground
{{#include ../doc-imports/src/design_essays/audio_slider.rs:audio_slider_sketch}}
```

It's fairly simple, and as mentioned above also opinionated in how the users have to store their values.
They must keep the value and disabled states in a `Mutable`, and the value must be of type `f64`.

Let's disregard user choice for the time being, and proceed by implementing the body of our simple slider.

## Implementing the slider

The slider needs to have a state representing if it's currently being manipulated or not. 
This can be stored in a simple `Mutable`, which we can just make at the top of our function:

```rust,no_run,noplayground
let button_state = Mutable::new(false);
```

To render the slider, we need two SVG rectangles:

The first rectangle represents the track in which the physical slider moves, and it can be drawn as a vertical narrow black rectangle.

The second rectangle represents the indicator knob, which is what the user will be moving along the slider to control the value.

For the knob, we can calculate its position as a signal derived from the value.
We will define it such that the lowest value of the range corresponds to the knob being at the bottom of the widget:

```rust,no_run,noplayground
let y_pos_signal = value.signal().map(move |v| {
    let value_scale = value_range.1 - value_range.0;
    let value_offset = value_range.0;

    let y_pos = 100.0 - 100.0 * (v - value_offset) / value_scale;

    y_pos.clamp(0.0, 100.0).to_string()
});
```

This makes rendering the knob relatively simple:

```rust,no_run,noplayground
svg!("rect", {
    .event(clone!(button_state, disabled => move |event: events::MouseDown| {
        button_state.set(!disabled.get());
    }))
    .attr_signal("y", y_pos_signal)
    .attr("width", "20")
    .attr("height", "10")
    .attr("fill", "gray")
    .attr("cursor", "pointer")
})
```

The event handler is responsible for starting the move operation when the button receives a `MouseDown` event.
Also notice that if the disabled state is true, we simply ignore the drag start.

We delegate releasing the drag state to a global event handler attached to the top level SVG element.
This has to be global so that we don't end up in a situation where the `MouseUp` event is received by a different element, causing the slider to be stuck in a move state!

The global handler is configured like his:

```rust,no_run,noplayground
.child(svg!("svg", {
    .attr("viewBox", "0 0 20 110")
    .apply(|builder| {
        builder.global_event(clone!(button_state => move |event: events::MouseUp| {
            button_state.set(false);
        }))
    })
    // ...
```

Now we need to handle mouse movement to change the value when the mouse moves inside the slider widgets screen area.

To convert this to values in the correct range, we first define a helper function to do the calculation:

```rust,no_run,noplayground
 let calculate_value = move |element: &SvgElement, offset_y: i32| -> f64 {
    let height = element.get_bounding_client_rect().height();
    let value_scale = value_range.1 - value_range.0;
    let value_offset = value_range.0;

    (value_offset + value_scale * (1.0 - offset_y as f64 / height)).clamp(value_range.0, value_range.1)
};
```

One interesting property of this function is that it expects a reference to the raw `SvgElement`.
The element handle lets us retrieve the bounding rectangle for the element, which is needed to convert the y offset from the drag event into a y percentage, which we need to calculate the correct value for the output.

To access the `SvgElement`, we have to use the `with_node!` macro discussed in [advanced element construction](../introduction/advanced_element_construction.md#accessing-the-real-dom-node):

```rust,no_run,noplayground
.with_node!(element => {
    .event(clone!(element, value, button_state => move |event: events::MouseMove| {
        if button_state.get() {
            value.set(calculate_value(&element, event.offset_y()));
        }
    }))
    // ...
})
```

The vertical bar is a simple rect, but for convenience, we'll allow clicking the vertical bar to instigate a drag operation as well:

```rust,no_run,noplayground
.child(svg!("rect", {
    .attr("x", "6")
    .attr("y", "5")
    .attr("width", "6")
    .attr("height", "100")
    .attr("cursor", "pointer")
    .event(clone!(element, button_state, disabled => move |event: events::MouseDown| {
        if !disabled.get() {
            button_state.set(true);
            value.set(calculate_value(&element, event.offset_y()))
        }
    }))
}))
```

The last piece of the puzzle is to have a visual indicator that the component is disabled.
We can solve this with a simple rect overlaying the entire component, with a slightly see-through grey tint:

```rust,no_run,noplayground
.child_signal(disabled.signal().map(|disabled| {
    if disabled {
        Some(svg!("rect", {
            .attr("width", "40px")
            .attr("height", "200px")
            .attr("opacity", "0.5")
            .attr("fill", "gray")
        }))
    } else {
        None
    }
}))
```

Finally, our naive implementation of the audio slider looks like this:

```rust,no_run,noplayground
{{#include ../doc-imports/src/design_essays/audio_slider.rs:audio_slider_final}}
```