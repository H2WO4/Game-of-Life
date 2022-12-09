```mermaid
stateDiagram-v2
	state Junc1 <<fork>>

	Hidden --> Junc1
	Junc1 --> Empty
	Junc1 --> Mined
	
	Hidden --> Flagged
	Flagged --> Hidden

```
