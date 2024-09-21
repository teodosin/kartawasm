Test project in preparation for Karta's web app implementation.

See LOG.md for the log of the development process. This is a throwaway project, so I'm not going to bother with a proper README.md. This is just my todo list. 

* [x] Get a basic wasm container with Bevy inside running in the browser.
* [ ] Write a script for building everything 
* [ ] Transmit a button press from the sveltekit UI to the wasm container.
* [ ] Make a svelte element react to an event from the wasm container.
* [ ] Try to use the wasi bindings from the wasi-fs-access repo to read a file from the filesystem.
* [ ] Try to create an agdb database and save it to the filesystem using the wasi bindings.
* [ ] Try to load the agdb database from the filesystem using the wasi bindings.
* [ ] Try to use the wasi bindings to create a node in the agdb database.
* [ ] Start a discussion in the AGDB repo if the above is proving difficult. 
* [ ] Declare this approach feasible or infeasible and move on. 