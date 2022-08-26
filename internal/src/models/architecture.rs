/*
 * Rigetti QCS API
 *
 * # Introduction  This is the documentation for the Rigetti QCS HTTP API.  You can find out more about Rigetti at [https://rigetti.com](https://rigetti.com), and also interact with QCS via the web at [https://qcs.rigetti.com](https://qcs.rigetti.com).  This API is documented in **OpenAPI format** and so is compatible with the dozens of language-specific client generators available [here](https://github.com/OpenAPITools/openapi-generator) and elsewhere on the web.  # Principles  This API follows REST design principles where appropriate, and otherwise an HTTP RPC paradigm. We adhere to the Google [API Improvement Proposals](https://google.aip.dev/general) where reasonable to provide a consistent, intuitive developer experience. HTTP response codes match their specifications, and error messages fit a common format.  # Authentication  All access to the QCS API requires OAuth2 authentication provided by Okta. You can request access [here](https://www.rigetti.com/get-quantum). Once you have a user account, you can download your access token from QCS [here](https://qcs.rigetti.com/auth/token).   That access token is valid for 24 hours after issuance. The value of `access_token` within the JSON file is the token used for authentication (don't use the entire JSON file).  Authenticate requests using the `Authorization` header and a `Bearer` prefix:  ``` curl --header \"Authorization: Bearer eyJraW...Iow\" ```  # Quantum Processor Access  Access to the quantum processors themselves is not yet provided directly by this HTTP API, but is instead performed over ZeroMQ/[rpcq](https://gitlab.com/rigetti/rpcq). Until that changes, we suggest using [pyquil](https://gitlab.com/rigetti/pyquil) to build and execute quantum programs via the Legacy API.  # Legacy API  Our legacy HTTP API remains accessible at https://forest-server.qcs.rigetti.com, and it shares a source of truth with this API's services. You can use either service with the same user account and means of authentication. We strongly recommend using the API documented here, as the legacy API is on the path to deprecation.
 *
 * The version of the OpenAPI document: 2020-07-31
 * Contact: support@rigetti.com
 * Generated by: https://openapi-generator.tech
 */

/// Architecture : Represents the logical underlying architecture of a quantum processor.  The architecture is defined in detail by the nodes and edges that constitute the quantum processor. This defines the set of all nodes that could be operated upon, and indicates to some approximation their physical layout. The main purpose of this is to support geometry calculations that are independent of the available operations, and rendering ISA-based information. Architecture layouts are defined by the `family`, as follows.  The \"Aspen\" family of quantum processor indicates a 2D planar grid layout of octagon unit cells. The `node_id` in this architecture is computed as :math:`100 p_y + 10 p_x + p_u` where :math:`p_y` is the zero-based Y position in the unit cell grid, :math:`p_x` is the zero-based X position in the unit cell grid, and :math:`p_u` is the zero-based position in the octagon unit cell and always ranges from 0 to 7. This scheme has a natural size limit of a 10x10 unit cell grid, which permits the architecture to scale up to 800 nodes.  Note that the operations that are actually available are defined entirely by `Operation` instances. The presence of a node or edge in the `Architecture` model provides no guarantee that any 1Q or 2Q operation will be available to users writing QUIL programs.

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Architecture {
    /// A list of all computational edges in the instruction set architecture.
    #[serde(rename = "edges")]
    pub edges: Vec<crate::models::Edge>,
    /// The architecture family. The nodes and edges conform to this family.
    #[serde(rename = "family")]
    pub family: Option<Box<crate::models::Family>>,
    /// A list of all computational nodes in the instruction set architecture.
    #[serde(rename = "nodes")]
    pub nodes: Vec<crate::models::Node>,
}

impl Architecture {
    /// Represents the logical underlying architecture of a quantum processor.  The architecture is defined in detail by the nodes and edges that constitute the quantum processor. This defines the set of all nodes that could be operated upon, and indicates to some approximation their physical layout. The main purpose of this is to support geometry calculations that are independent of the available operations, and rendering ISA-based information. Architecture layouts are defined by the `family`, as follows.  The \"Aspen\" family of quantum processor indicates a 2D planar grid layout of octagon unit cells. The `node_id` in this architecture is computed as :math:`100 p_y + 10 p_x + p_u` where :math:`p_y` is the zero-based Y position in the unit cell grid, :math:`p_x` is the zero-based X position in the unit cell grid, and :math:`p_u` is the zero-based position in the octagon unit cell and always ranges from 0 to 7. This scheme has a natural size limit of a 10x10 unit cell grid, which permits the architecture to scale up to 800 nodes.  Note that the operations that are actually available are defined entirely by `Operation` instances. The presence of a node or edge in the `Architecture` model provides no guarantee that any 1Q or 2Q operation will be available to users writing QUIL programs.
    pub fn new(
        edges: Vec<crate::models::Edge>,
        family: Option<crate::models::Family>,
        nodes: Vec<crate::models::Node>,
    ) -> Architecture {
        Architecture {
            edges,
            family: if let Some(x) = family {
                Some(Box::new(x))
            } else {
                None
            },
            nodes,
        }
    }
}
