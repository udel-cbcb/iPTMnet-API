class QuerySubstrate:

    schema = {
        "type" : "object",
        "properties": {
            "substrate_ac": {"type":"string"},
            "site_residue":{"type":"string"},
            "site_position":{"type":"string"}
        }
    }

    def __init__(self,substrate_ac,site_residue,site_position):
        self.substrate_ac = substrate_ac
        self.site_residue = site_residue
        self.site_position = site_position