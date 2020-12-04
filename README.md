# RS-GFAHANDLEGRAPH
rs-gfahandlegraph it's a library to manipulate GFA files as HandleGraph graphs.\
This library can work with either GFA version 1 and version 2.

## HOW IT WORKS
rs-gfahandlegraph performs 2 main activity:
- read and parsing a file to ensure it is in a valid format (either gfa1 or 2):
  this operation will create an intermediate object with the same logical structure of the file 
  were **ONLY** the fields that will be later used to create the HandleGraph object will be saved.
- create the handlegraph from the fresh parsed file:
  using the newly created gfa-object the HandleGraph will be created using the values stored in the S-, L-(or E-) and P-(or O-)
  fields.
  
Moreover, this library let the user perform all the operation they need to do over the graph, from create it from a file or create it 
manually, inserting each node, links and paths by hands.\
It allows to modify the graph removing all kind of items, like nodes, edges, entire paths or specific nodes from specific paths.\
Of course, on top of that it allows the user to modify the element in the graph.
\
Finally, when all the operations on the graph are terminated, the library allows the user to save the graph back as a GFA format 1
or 2, back on a file.


