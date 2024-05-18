app "hello-world"
    packages { pf: "https://github.com/roc-lang/basic-cli/releases/download/0.8.1/x8URkvfyi9I0QhmVG98roKBUs_AZRkLFwFJVJ3942YA.tar.br" }
    imports [pf.Stdout, pf.Task { Task }]
    provides [main] to pf

main = Stdout.line fullName "Hello, " "Johnny" "Cash"

fullName : Str, Str -> Str
fullName = \firstName, lastName ->
    "$(firstName) $(lastName)"

fullName2 = \firstName, lastName ->
    "$(firstName) $(lastName)"



##
## 1. The loop:
##    - Checks for input commands
##    - Runs every system, producing a reduced World and using the reduced World for the system that follows up
##    - Then checks if there is an exit event from one of the systems
##    - Then checks if it there is an event of being terminated with Ctrl+C

loop = \world, systems, events, nextCycleSignal ->
    shouldExit = List.any events \event ->
        match event with
            Exit -> True
            _ -> False

    result = when shouldExit is
        True -> CycleOutput::Terminate()
        False -> CycleOutput::Continue(reduceWorld world systems)
    
    nextRun <- nextCycleSignal |> Task.wait

    result

reduceWorld = \world, systems ->
    output_commands = systems |> List.walk world \world, system -> world |> system
    output_commands

