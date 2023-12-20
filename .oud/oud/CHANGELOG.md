# changelog

## upcoming

## v0.0.0-rolling.3

In this not-fully-released release, we have made project windows display generated metadata from a datasource lol

- added support for `1.19.4-pre4`, `1.19.4-rc1`, and `1.19.4-rc2`
- macOS releases now signed!
- app knows its _real_ version instead of the manifest declared one, will be used in auto updates, about screen, and probably others
  - this is because wix, windows' packaging toolkit, doesn't support proper semver, and i'm not about to rewrite my entire build/publish pipeline just for this. Especially since nsis targets are on the way (maybe???) and that supports semver proper.
- recents list is now actually a recents list, instead of static nonsense fake paths, and actually works (if you click an entry it opens/focuses the window)
- correct version will now reflect in the app (For Real this time)
- builds not made with a release will have `-dev` at the end of the version
- created docs project, will fill with documentation soon&trade;, its currently empty with a _tiny_ bit of filler content lol
- "No recent projects" message when there is none
- played with creating my own i18n module (existing solutions aren't good enough for what i want), shelved for now its relatively low priority
- created site project, nothing in it for now h
- use purple coloured selection (uwu)
- fix title bar weirdly resizing itself on window resize
- internally allow creating meta objects for datasource/etc without specifying an mc version, and allow to create version specifics from there
- `default` is now specified on Texture using a string (that is the same name as the dir of the option) instead of `default: true` on the default option. This way, you cannot specify multiple defaults by accident, and it makes it easier to write code to handle too.
- make project windows not transparent
- filter invalid mc versions specified in a Version and raise an error

## v0.0.0-rolling.2

- ~~correct version will now reflect in the app~~ nope! I overlooked something, oops >.<
- created changelog (hopefully will be kept updated??)
- change soon text for "New Project" button from "Coming Soon&trade;" to "Soon&trade;", makes it fit in one line
- add documentation, changelog buttons to welcome
- add about screen (currently only gives basic info, version, copyright, license, github link)
- change title / tagline font on welcome screen to permanent marker

## v0.0.0-rolling.1

this was a test release, for setting up automated releases. App doesn't work quite yet :p

- `lib`'s very basic and essential functionality is complete, meaning that the essential advertised features should be working once hooked up to the frontend! (but should still be considered alpha, needs more testing)
- added welcome screen, allow to open multiwindow / multiproject functionality
  - recents page is a hardcoded bunch of random paths
  - the window that opens when you open a project does absolutely nothing but display text that was used as a scrolling test
- no dark mode support yet h
