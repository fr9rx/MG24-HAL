## Instruction (follow stricly)

1. When a feature is add its commicted to git with a clean commit message and commited to github
2. When there is 2-3 features added republish to crates.io
3. DONT USE the patteren let _ = in any of the internal HAL code or the user API its considered as a illegal thing
4. Before commiting or publishing to crates.io ensure there is no erros or warnings left when building the HAL it makes the HAL look unprofessinal
5. Examples should be folders in examples folder as a independet project so any user can clone and not clone the whole repo
6. Avoid AI indications in the code or the documenation
7. Document Every feature so it can be read back at any moment and make the code understandable to edit
8. Write Any feature with scalabale archteticure
9. Every feature added its API must be documented in 2 files one for human API for all the features in the HAL and one for Agents to get every aspect for the API as a single source of truth file for the API when writing code using this HAL
10. Maintain a CHANELOG.md file that has every commit history with date and changes in files and features added
11. Add all the documenation into the docs folder and put files that have a copyright and cant be commited to github internal_docs folder and ensure its in .gitignore
12. Maintain a AI_CONTRIBUTING_POLICY.md file and CONTRIBUTIIN_POLICY.md file
