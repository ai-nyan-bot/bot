- A condition must at least contain of one compare for the engine to process it - otherwise every pair might match

Ensure that blocks are ingested in order - otherwise the aggregator might just skip entire blocks
Workaround: tell aggregation pipeline until which slot can be aggregated - this enables parallel ingestion of blocks