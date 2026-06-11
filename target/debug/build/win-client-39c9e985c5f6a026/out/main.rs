# [allow (non_snake_case)] # [allow (non_camel_case_types)] # [allow (clippy :: style)] # [allow (clippy :: complexity)] # [allow (unused_braces , unused_parens)] # [allow (clippy :: erasing_op)] # [allow (clippy :: approx_constant)] # [allow (clippy :: eq_op)] # [allow (clippy :: cmp_owned)] # [allow (clippy :: redundant_clone)] mod slint_generatedMainWindow {
     use slint :: private_unstable_api :: re_exports :: * ;
     use slint :: private_unstable_api :: re_exports as sp ;
     # [derive (Default , PartialEq , Debug , Clone)] pub struct r#TextStyle {
         pub r#font_size : f32 , pub r#font_weight : i32 }
     # [derive (sp :: FieldOffsets , Default)] # [const_field_offset (sp :: const_field_offset)] # [repr (C)] # [pin] struct InnerColorSchemeSelector_37 {
         r#dark_color_scheme : sp :: Property < bool > , root : sp :: OnceCell < sp :: VWeak < sp :: ComponentVTable , InnerMainWindow >> , }
     impl InnerColorSchemeSelector_37 {
         fn new () -> :: core :: pin :: Pin < sp :: Rc < Self >> {
             sp :: Rc :: pin (Self :: default ()) }
         fn init (self : :: core :: pin :: Pin < sp :: Rc < Self >> , root : & sp :: VRc < sp :: ComponentVTable , InnerMainWindow >) {
             # ! [allow (unused)] self . root . set (VRc :: downgrade (root)) ;
             let self_rc = self ;
             let _self = self_rc . as_ref () ;
             {
                 slint :: private_unstable_api :: set_property_binding ({
                     * & InnerColorSchemeSelector_37 :: FIELD_OFFSETS . r#dark_color_scheme }
                 . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_ref () ;
                     (sp :: WindowInner :: from_pub ((& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()) . window ()) . dark_color_scheme ()) as _ }
                ) ;
                 }
             }
         }
     # [derive (sp :: FieldOffsets , Default)] # [const_field_offset (sp :: const_field_offset)] # [repr (C)] # [pin] struct InnerPalette_38 {
         r#accent_control_border : sp :: Property < slint :: Brush > , r#control_border : sp :: Property < slint :: Brush > , r#text_control_border : sp :: Property < slint :: Brush > , root : sp :: OnceCell < sp :: VWeak < sp :: ComponentVTable , InnerMainWindow >> , }
     impl InnerPalette_38 {
         fn new () -> :: core :: pin :: Pin < sp :: Rc < Self >> {
             sp :: Rc :: pin (Self :: default ()) }
         fn init (self : :: core :: pin :: Pin < sp :: Rc < Self >> , root : & sp :: VRc < sp :: ComponentVTable , InnerMainWindow >) {
             # ! [allow (unused)] self . root . set (VRc :: downgrade (root)) ;
             let self_rc = self ;
             let _self = self_rc . as_ref () ;
             {
                 slint :: private_unstable_api :: set_property_binding ({
                     * & InnerPalette_38 :: FIELD_OFFSETS . r#accent_control_border }
                 . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_ref () ;
                     (if InnerColorSchemeSelector_37 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . root . get () . unwrap () . upgrade () . unwrap () . globals . global_ColorSchemeSelector_37 . as_ref ()) . get () {
                         slint :: Brush :: LinearGradient (sp :: LinearGradientBrush :: new (180f64 as _ , [sp :: GradientStop {
                             color : sp :: Color :: from_argb_encoded (352321535f64 as u32) , position : ((90.67f64 as f64) * (0.01f64 as f64)) as _ }
                         , sp :: GradientStop {
                             color : sp :: Color :: from_argb_encoded (603979776f64 as u32) , position : ((100f64 as f64) * (0.01f64 as f64)) as _ }
                        ])) }
                     else {
                         (slint :: Brush :: LinearGradient (sp :: LinearGradientBrush :: new (180f64 as _ , [sp :: GradientStop {
                             color : sp :: Color :: from_argb_encoded (352321535f64 as u32) , position : ((90.67f64 as f64) * (0.01f64 as f64)) as _ }
                         , sp :: GradientStop {
                             color : sp :: Color :: from_argb_encoded (1711276032f64 as u32) , position : ((100f64 as f64) * (0.01f64 as f64)) as _ }
                        ]))) as _ }
                    ) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ({
                     * & InnerPalette_38 :: FIELD_OFFSETS . r#control_border }
                 . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_ref () ;
                     (if InnerColorSchemeSelector_37 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . root . get () . unwrap () . upgrade () . unwrap () . globals . global_ColorSchemeSelector_37 . as_ref ()) . get () {
                         slint :: Brush :: LinearGradient (sp :: LinearGradientBrush :: new (180f64 as _ , [sp :: GradientStop {
                             color : sp :: Color :: from_argb_encoded (402653183f64 as u32) , position : ((0f64 as f64) * (0.01f64 as f64)) as _ }
                         , sp :: GradientStop {
                             color : sp :: Color :: from_argb_encoded (301989888f64 as u32) , position : ((8.33f64 as f64) * (0.01f64 as f64)) as _ }
                        ])) }
                     else {
                         (slint :: Brush :: LinearGradient (sp :: LinearGradientBrush :: new (180f64 as _ , [sp :: GradientStop {
                             color : sp :: Color :: from_argb_encoded (251658240f64 as u32) , position : ((90.58f64 as f64) * (0.01f64 as f64)) as _ }
                         , sp :: GradientStop {
                             color : sp :: Color :: from_argb_encoded (687865856f64 as u32) , position : ((100f64 as f64) * (0.01f64 as f64)) as _ }
                        ]))) as _ }
                    ) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ({
                     * & InnerPalette_38 :: FIELD_OFFSETS . r#text_control_border }
                 . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_ref () ;
                     (if InnerColorSchemeSelector_37 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . root . get () . unwrap () . upgrade () . unwrap () . globals . global_ColorSchemeSelector_37 . as_ref ()) . get () {
                         slint :: Brush :: LinearGradient (sp :: LinearGradientBrush :: new (180f64 as _ , [sp :: GradientStop {
                             color : sp :: Color :: from_argb_encoded (352321535f64 as u32) , position : ((99.98f64 as f64) * (0.01f64 as f64)) as _ }
                         , sp :: GradientStop {
                             color : sp :: Color :: from_argb_encoded (2332033023f64 as u32) , position : ((100f64 as f64) * (0.01f64 as f64)) as _ }
                         , sp :: GradientStop {
                             color : sp :: Color :: from_argb_encoded (2332033023f64 as u32) , position : ((100f64 as f64) * (0.01f64 as f64)) as _ }
                        ])) }
                     else {
                         (slint :: Brush :: LinearGradient (sp :: LinearGradientBrush :: new (180f64 as _ , [sp :: GradientStop {
                             color : sp :: Color :: from_argb_encoded (251658240f64 as u32) , position : ((99.99f64 as f64) * (0.01f64 as f64)) as _ }
                         , sp :: GradientStop {
                             color : sp :: Color :: from_argb_encoded (1929379840f64 as u32) , position : ((100f64 as f64) * (0.01f64 as f64)) as _ }
                         , sp :: GradientStop {
                             color : sp :: Color :: from_argb_encoded (1929379840f64 as u32) , position : ((100f64 as f64) * (0.01f64 as f64)) as _ }
                        ]))) as _ }
                    ) as _ }
                ) ;
                 }
             }
         }
     # [derive (sp :: FieldOffsets , Default)] # [const_field_offset (sp :: const_field_offset)] # [repr (C)] # [pin] struct InnerFocusBorder_root_23 {
         r#root_23 : sp :: r#BorderRectangle , r#rectangle_24 : sp :: r#BorderRectangle , self_weak : sp :: OnceCell < sp :: VWeakMapped < sp :: ComponentVTable , InnerFocusBorder_root_23 >> , root : sp :: OnceCell < sp :: VWeak < sp :: ComponentVTable , InnerMainWindow >> , tree_index : :: core :: cell :: Cell < u32 > , tree_index_of_first_child : :: core :: cell :: Cell < u32 > , }
     impl InnerFocusBorder_root_23 {
         pub fn init (self_rc : sp :: VRcMapped < sp :: ComponentVTable , Self > , root : & sp :: VRc < sp :: ComponentVTable , InnerMainWindow > , tree_index : u32 , tree_index_of_first_child : u32) {
             # ! [allow (unused)] let _self = self_rc . as_pin_ref () ;
             _self . self_weak . set (VRcMapped :: downgrade (& self_rc)) ;
             _self . root . set (VRc :: downgrade (root)) ;
             _self . tree_index . set (tree_index) ;
             _self . tree_index_of_first_child . set (tree_index_of_first_child) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerFocusBorder_root_23 :: FIELD_OFFSETS . r#root_23 }
                 + r#BorderRectangle :: FIELD_OFFSETS . r#border_color) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: Brush :: SolidColor (if InnerColorSchemeSelector_37 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . root . get () . unwrap () . upgrade () . unwrap () . globals . global_ColorSchemeSelector_37 . as_ref ()) . get () {
                         sp :: Color :: from_argb_encoded (4294967295f64 as u32) }
                     else {
                         (sp :: Color :: from_argb_encoded (3858759680f64 as u32)) as _ }
                    )) as _ }
                ) ;
                 }
             ({
                 * & InnerFocusBorder_root_23 :: FIELD_OFFSETS . r#root_23 }
             + r#BorderRectangle :: FIELD_OFFSETS . r#border_width) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (2f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerFocusBorder_root_23 :: FIELD_OFFSETS . r#rectangle_24 }
                 + r#BorderRectangle :: FIELD_OFFSETS . r#border_color) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: Brush :: SolidColor (if InnerColorSchemeSelector_37 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . root . get () . unwrap () . upgrade () . unwrap () . globals . global_ColorSchemeSelector_37 . as_ref ()) . get () {
                         sp :: Color :: from_argb_encoded (3003121664f64 as u32) }
                     else {
                         (sp :: Color :: from_argb_encoded (4294967295f64 as u32)) as _ }
                    )) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerFocusBorder_root_23 :: FIELD_OFFSETS . r#rectangle_24 }
                 + r#BorderRectangle :: FIELD_OFFSETS . r#border_radius) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (((({
                         * & InnerFocusBorder_root_23 :: FIELD_OFFSETS . r#root_23 }
                     + r#BorderRectangle :: FIELD_OFFSETS . r#border_radius) . apply_pin (_self) . get () . get () as f64) - (2f64 as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & InnerFocusBorder_root_23 :: FIELD_OFFSETS . r#rectangle_24 }
             + r#BorderRectangle :: FIELD_OFFSETS . r#border_width) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (1f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerFocusBorder_root_23 :: FIELD_OFFSETS . r#rectangle_24 }
                 + r#BorderRectangle :: FIELD_OFFSETS . r#height) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (((({
                         * & InnerFocusBorder_root_23 :: FIELD_OFFSETS . r#root_23 }
                     + r#BorderRectangle :: FIELD_OFFSETS . r#height) . apply_pin (_self) . get () . get () as f64) - (4f64 as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerFocusBorder_root_23 :: FIELD_OFFSETS . r#rectangle_24 }
                 + r#BorderRectangle :: FIELD_OFFSETS . r#width) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (((({
                         * & InnerFocusBorder_root_23 :: FIELD_OFFSETS . r#root_23 }
                     + r#BorderRectangle :: FIELD_OFFSETS . r#width) . apply_pin (_self) . get () . get () as f64) - (4f64 as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & InnerFocusBorder_root_23 :: FIELD_OFFSETS . r#rectangle_24 }
             + r#BorderRectangle :: FIELD_OFFSETS . r#x) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (2f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 * & InnerFocusBorder_root_23 :: FIELD_OFFSETS . r#rectangle_24 }
             + r#BorderRectangle :: FIELD_OFFSETS . r#y) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (2f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 * & InnerFocusBorder_root_23 :: FIELD_OFFSETS . r#root_23 }
             + r#BorderRectangle :: FIELD_OFFSETS . r#background) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerFocusBorder_root_23 :: FIELD_OFFSETS . r#root_23 }
             + r#BorderRectangle :: FIELD_OFFSETS . r#border_width) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerFocusBorder_root_23 :: FIELD_OFFSETS . r#root_23 }
             + r#BorderRectangle :: FIELD_OFFSETS . r#x) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerFocusBorder_root_23 :: FIELD_OFFSETS . r#root_23 }
             + r#BorderRectangle :: FIELD_OFFSETS . r#y) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerFocusBorder_root_23 :: FIELD_OFFSETS . r#rectangle_24 }
             + r#BorderRectangle :: FIELD_OFFSETS . r#background) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerFocusBorder_root_23 :: FIELD_OFFSETS . r#rectangle_24 }
             + r#BorderRectangle :: FIELD_OFFSETS . r#border_width) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerFocusBorder_root_23 :: FIELD_OFFSETS . r#rectangle_24 }
             + r#BorderRectangle :: FIELD_OFFSETS . r#x) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerFocusBorder_root_23 :: FIELD_OFFSETS . r#rectangle_24 }
             + r#BorderRectangle :: FIELD_OFFSETS . r#y) . apply_pin (_self) . set_constant () ;
             }
         pub fn user_init (self_rc : sp :: VRcMapped < sp :: ComponentVTable , Self >) {
             let _self = self_rc . as_pin_ref () ;
             }
         fn visit_dynamic_children (self : :: core :: pin :: Pin < & Self > , dyn_index : usize , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut) -> sp :: VisitChildrenResult {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn layout_info (self : :: core :: pin :: Pin < & Self > , orientation : sp :: Orientation) -> sp :: LayoutInfo {
             # ! [allow (unused)] let _self = self ;
             match orientation {
                 sp :: Orientation :: Horizontal => sp :: Item :: layout_info (({
                     * & InnerFocusBorder_root_23 :: FIELD_OFFSETS . r#root_23 }
                ) . apply_pin (_self) , sp :: Orientation :: Horizontal , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ())) , sp :: Orientation :: Vertical => sp :: Item :: layout_info (({
                     * & InnerFocusBorder_root_23 :: FIELD_OFFSETS . r#root_23 }
                ) . apply_pin (_self) , sp :: Orientation :: Vertical , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ())) , }
             }
         fn subtree_range (self : :: core :: pin :: Pin < & Self > , dyn_index : usize) -> sp :: IndexRange {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn subtree_component (self : :: core :: pin :: Pin < & Self > , dyn_index : usize , subtree_index : usize , result : & mut sp :: ComponentWeak) {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             ;
             }
         fn index_property (self : :: core :: pin :: Pin < & Self >) -> usize {
             # ! [allow (unused)] let _self = self ;
             core :: usize :: MAX }
         fn accessible_role (self : :: core :: pin :: Pin < & Self > , index : usize) -> sp :: AccessibleRole {
             # ! [allow (unused)] let _self = self ;
             match index {
                 _ => AccessibleRole :: default () , }
             }
         fn accessible_string_property (self : :: core :: pin :: Pin < & Self > , index : usize , what : sp :: AccessibleStringProperty ,) -> sp :: SharedString {
             # ! [allow (unused)] let _self = self ;
             match (index , what) {
                 _ => Default :: default () , }
             }
         }
     # [derive (sp :: FieldOffsets , Default)] # [const_field_offset (sp :: const_field_offset)] # [repr (C)] # [pin] struct InnerButton_root_25 {
         r#root_25 : sp :: r#Empty , r#i_background_26 : sp :: r#BorderRectangle , r#i_border_27 : sp :: r#BorderRectangle , r#i_touch_area_33 : sp :: r#TouchArea , r#i_focus_scope_34 : sp :: r#FocusScope , r#root_25_checkable : sp :: Property < bool > , r#root_25_checked : sp :: Property < bool > , r#root_25_has_focus : sp :: Property < bool > , r#root_25_i_layout_28_layout_cache : sp :: Property < sp :: SharedVector < sp :: Coord , > > , r#root_25_i_layout_28_layoutinfo_h : sp :: Property < r#LayoutInfo > , r#root_25_i_layout_28_layoutinfo_v : sp :: Property < r#LayoutInfo > , r#root_25_icon : sp :: Property < sp :: Image > , r#root_25_pressed : sp :: Property < bool > , r#root_25_primary : sp :: Property < bool > , r#root_25_state : sp :: Property < i32 > , r#root_25_text : sp :: Property < sp :: SharedString > , r#root_25_text_color : sp :: Property < slint :: Brush > , r#root_25_clicked : sp :: Callback < () , () > , repeater0 : sp :: Repeater < InnerComponent_image_29 > , repeater1 : sp :: Repeater < InnerComponent_text_31 > , repeater2 : sp :: Repeater < InnerComponent_focusborder_35 > , self_weak : sp :: OnceCell < sp :: VWeakMapped < sp :: ComponentVTable , InnerButton_root_25 >> , root : sp :: OnceCell < sp :: VWeak < sp :: ComponentVTable , InnerMainWindow >> , tree_index : :: core :: cell :: Cell < u32 > , tree_index_of_first_child : :: core :: cell :: Cell < u32 > , }
     impl InnerButton_root_25 {
         pub fn init (self_rc : sp :: VRcMapped < sp :: ComponentVTable , Self > , root : & sp :: VRc < sp :: ComponentVTable , InnerMainWindow > , tree_index : u32 , tree_index_of_first_child : u32) {
             # ! [allow (unused)] let _self = self_rc . as_pin_ref () ;
             _self . self_weak . set (VRcMapped :: downgrade (& self_rc)) ;
             _self . root . set (VRc :: downgrade (root)) ;
             _self . tree_index . set (tree_index) ;
             _self . tree_index_of_first_child . set (tree_index_of_first_child) ;
             _self . repeater0 . set_model_binding ({
                 let self_weak = sp :: VRcMapped :: downgrade (& self_rc) ;
                 move || {
                     let self_rc = self_weak . upgrade () . unwrap () ;
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: ModelRc :: new ({
                         let r#tmp_root_25_icon = sp :: Image :: default () ;
                         ;
                         (((((r#tmp_root_25_icon . clone () . size ()) . r#width as f64) > (0f64 as f64))) && ((((r#tmp_root_25_icon . clone () . size ()) . r#height as f64) > (0f64 as f64)))) }
                     as bool)) as _ }
                 }
            ) ;
             _self . repeater1 . set_model_binding ({
                 let self_weak = sp :: VRcMapped :: downgrade (& self_rc) ;
                 move || {
                     let self_rc = self_weak . upgrade () . unwrap () ;
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: ModelRc :: new (((({
                         * & InnerButton_root_25 :: FIELD_OFFSETS . r#root_25_text }
                    ) . apply_pin (_self) . get ()) != (sp :: SharedString :: from (""))) as bool)) as _ }
                 }
            ) ;
             _self . repeater2 . set_model_binding ({
                 let self_weak = sp :: VRcMapped :: downgrade (& self_rc) ;
                 move || {
                     let self_rc = self_weak . upgrade () . unwrap () ;
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: ModelRc :: new (((({
                         * & InnerButton_root_25 :: FIELD_OFFSETS . r#root_25_has_focus }
                    ) . apply_pin (_self) . get ()) && (({
                         * & InnerButton_root_25 :: FIELD_OFFSETS . r#i_focus_scope_34 }
                     + r#FocusScope :: FIELD_OFFSETS . r#enabled) . apply_pin (_self) . get ())) as bool)) as _ }
                 }
            ) ;
             Property :: link_two_way (({
                 * & InnerButton_root_25 :: FIELD_OFFSETS . r#i_touch_area_33 }
             + r#TouchArea :: FIELD_OFFSETS . r#enabled) . apply_pin (_self) , ({
                 * & InnerButton_root_25 :: FIELD_OFFSETS . r#i_focus_scope_34 }
             + r#FocusScope :: FIELD_OFFSETS . r#enabled) . apply_pin (_self)) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerButton_root_25 :: FIELD_OFFSETS . r#root_25_has_focus }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (({
                         * & InnerButton_root_25 :: FIELD_OFFSETS . r#i_focus_scope_34 }
                     + r#FocusScope :: FIELD_OFFSETS . r#has_focus) . apply_pin (_self) . get ()) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerButton_root_25 :: FIELD_OFFSETS . r#root_25_i_layout_28_layout_cache }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ({
                         let mut r#repeated_indices = [0u32 ;
                         2 * 2usize] ;
                         let mut items_vec = sp :: Vec :: with_capacity (0usize + _self . repeater0 . len () + _self . repeater1 . len ()) ;
                         InnerButton_root_25 :: FIELD_OFFSETS . repeater0 . apply_pin (_self) . ensure_updated (|| {
                             InnerComponent_image_29 :: new (_self . self_weak . get () . unwrap () . clone ()) . into () }
                        ) ;
                         let internal_vec = _self . repeater0 . components_vec () ;
                         r#repeated_indices [0usize * 2] = items_vec . len () as u32 ;
                         r#repeated_indices [0usize * 2 + 1] = internal_vec . len () as u32 ;
                         for sub_comp in & internal_vec {
                             items_vec . push (sub_comp . as_pin_ref () . box_layout_data (sp :: Orientation :: Horizontal)) }
                         InnerButton_root_25 :: FIELD_OFFSETS . repeater1 . apply_pin (_self) . ensure_updated (|| {
                             InnerComponent_text_31 :: new (_self . self_weak . get () . unwrap () . clone ()) . into () }
                        ) ;
                         let internal_vec = _self . repeater1 . components_vec () ;
                         r#repeated_indices [1usize * 2] = items_vec . len () as u32 ;
                         r#repeated_indices [1usize * 2 + 1] = internal_vec . len () as u32 ;
                         for sub_comp in & internal_vec {
                             items_vec . push (sub_comp . as_pin_ref () . box_layout_data (sp :: Orientation :: Horizontal)) }
                         let r#cells = sp :: Slice :: from_slice (& items_vec) ;
                         let r#repeated_indices = sp :: Slice :: from_slice (& r#repeated_indices) ;
                         r#solve_box_layout (& r#BoxLayoutData {
                             r#alignment : sp :: r#LayoutAlignment :: r#Center as _ , r#cells : r#cells . clone () as _ , r#padding : {
                                 let mut the_struct = r#Padding :: default () ;
                                 the_struct . r#begin = 12f64 as _ ;
                                 the_struct . r#end = 12f64 as _ ;
                                 the_struct }
                             as _ , r#size : ({
                                 * & InnerButton_root_25 :: FIELD_OFFSETS . r#i_background_26 }
                             + r#BorderRectangle :: FIELD_OFFSETS . r#width) . apply_pin (_self) . get () . get () as _ , r#spacing : 4f64 as _ , }
                         as _ , r#repeated_indices . clone () as _) }
                    ) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerButton_root_25 :: FIELD_OFFSETS . r#root_25_i_layout_28_layoutinfo_h }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ({
                         let mut items_vec = sp :: Vec :: with_capacity (0usize + _self . repeater0 . len () + _self . repeater1 . len ()) ;
                         InnerButton_root_25 :: FIELD_OFFSETS . repeater0 . apply_pin (_self) . ensure_updated (|| {
                             InnerComponent_image_29 :: new (_self . self_weak . get () . unwrap () . clone ()) . into () }
                        ) ;
                         let internal_vec = _self . repeater0 . components_vec () ;
                         for sub_comp in & internal_vec {
                             items_vec . push (sub_comp . as_pin_ref () . box_layout_data (sp :: Orientation :: Horizontal)) }
                         InnerButton_root_25 :: FIELD_OFFSETS . repeater1 . apply_pin (_self) . ensure_updated (|| {
                             InnerComponent_text_31 :: new (_self . self_weak . get () . unwrap () . clone ()) . into () }
                        ) ;
                         let internal_vec = _self . repeater1 . components_vec () ;
                         for sub_comp in & internal_vec {
                             items_vec . push (sub_comp . as_pin_ref () . box_layout_data (sp :: Orientation :: Horizontal)) }
                         let r#cells = sp :: Slice :: from_slice (& items_vec) ;
                         r#box_layout_info (r#cells . clone () as _ , 4f64 as _ , & {
                             let mut the_struct = r#Padding :: default () ;
                             the_struct . r#begin = 12f64 as _ ;
                             the_struct . r#end = 12f64 as _ ;
                             the_struct }
                         as _ , sp :: r#LayoutAlignment :: r#Center as _) }
                    ) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerButton_root_25 :: FIELD_OFFSETS . r#root_25_i_layout_28_layoutinfo_v }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ({
                         let mut items_vec = sp :: Vec :: with_capacity (0usize + _self . repeater0 . len () + _self . repeater1 . len ()) ;
                         InnerButton_root_25 :: FIELD_OFFSETS . repeater0 . apply_pin (_self) . ensure_updated (|| {
                             InnerComponent_image_29 :: new (_self . self_weak . get () . unwrap () . clone ()) . into () }
                        ) ;
                         let internal_vec = _self . repeater0 . components_vec () ;
                         for sub_comp in & internal_vec {
                             items_vec . push (sub_comp . as_pin_ref () . box_layout_data (sp :: Orientation :: Vertical)) }
                         InnerButton_root_25 :: FIELD_OFFSETS . repeater1 . apply_pin (_self) . ensure_updated (|| {
                             InnerComponent_text_31 :: new (_self . self_weak . get () . unwrap () . clone ()) . into () }
                        ) ;
                         let internal_vec = _self . repeater1 . components_vec () ;
                         for sub_comp in & internal_vec {
                             items_vec . push (sub_comp . as_pin_ref () . box_layout_data (sp :: Orientation :: Vertical)) }
                         let r#cells = sp :: Slice :: from_slice (& items_vec) ;
                         r#box_layout_info_ortho (r#cells . clone () as _ , & {
                             let mut the_struct = r#Padding :: default () ;
                             the_struct . r#begin = 5f64 as _ ;
                             the_struct . r#end = 5f64 as _ ;
                             the_struct }
                         as _) }
                    ) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerButton_root_25 :: FIELD_OFFSETS . r#root_25_pressed }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (((({
                         * & InnerButton_root_25 :: FIELD_OFFSETS . r#i_focus_scope_34 }
                     + r#FocusScope :: FIELD_OFFSETS . r#enabled) . apply_pin (_self) . get ()) && (({
                         * & InnerButton_root_25 :: FIELD_OFFSETS . r#i_touch_area_33 }
                     + r#TouchArea :: FIELD_OFFSETS . r#pressed) . apply_pin (_self) . get ()))) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerButton_root_25 :: FIELD_OFFSETS . r#root_25_state }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (if ! ({
                         * & InnerButton_root_25 :: FIELD_OFFSETS . r#i_focus_scope_34 }
                     + r#FocusScope :: FIELD_OFFSETS . r#enabled) . apply_pin (_self) . get () {
                         1f64 }
                     else {
                         (if ({
                             * & InnerButton_root_25 :: FIELD_OFFSETS . r#root_25_pressed }
                        ) . apply_pin (_self) . get () {
                             2f64 }
                         else {
                             (if ({
                                 * & InnerButton_root_25 :: FIELD_OFFSETS . r#i_touch_area_33 }
                             + r#TouchArea :: FIELD_OFFSETS . r#has_hover) . apply_pin (_self) . get () {
                                 3f64 }
                             else {
                                 (if ({
                                     * & InnerButton_root_25 :: FIELD_OFFSETS . r#root_25_checked }
                                ) . apply_pin (_self) . get () {
                                     4f64 }
                                 else {
                                     (0f64) as _ }
                                ) as _ }
                            ) as _ }
                        ) as _ }
                    ) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerButton_root_25 :: FIELD_OFFSETS . r#root_25_text_color }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ({
                         let r#tmp_root_25_state = ({
                             * & InnerButton_root_25 :: FIELD_OFFSETS . r#root_25_state }
                        ) . apply_pin (_self) . get () ;
                         ;
                         if ((r#tmp_root_25_state . clone () as f64) == (1f64 as f64)) {
                             if ((({
                                 * & InnerButton_root_25 :: FIELD_OFFSETS . r#root_25_primary }
                            ) . apply_pin (_self) . get ()) || (({
                                 * & InnerButton_root_25 :: FIELD_OFFSETS . r#root_25_checked }
                            ) . apply_pin (_self) . get ())) {
                                 slint :: Brush :: SolidColor (if InnerColorSchemeSelector_37 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . root . get () . unwrap () . upgrade () . unwrap () . globals . global_ColorSchemeSelector_37 . as_ref ()) . get () {
                                     sp :: Color :: from_argb_encoded (2281701375f64 as u32) }
                                 else {
                                     (sp :: Color :: from_argb_encoded (4294967295f64 as u32)) as _ }
                                ) }
                             else {
                                 (slint :: Brush :: SolidColor (if InnerColorSchemeSelector_37 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . root . get () . unwrap () . upgrade () . unwrap () . globals . global_ColorSchemeSelector_37 . as_ref ()) . get () {
                                     sp :: Color :: from_argb_encoded (1593835519f64 as u32) }
                                 else {
                                     (sp :: Color :: from_argb_encoded (1577058304f64 as u32)) as _ }
                                )) as _ }
                             }
                         else {
                             (if ((r#tmp_root_25_state . clone () as f64) == (2f64 as f64)) {
                                 if ((({
                                     * & InnerButton_root_25 :: FIELD_OFFSETS . r#root_25_primary }
                                ) . apply_pin (_self) . get ()) || (({
                                     * & InnerButton_root_25 :: FIELD_OFFSETS . r#root_25_checked }
                                ) . apply_pin (_self) . get ())) {
                                     slint :: Brush :: SolidColor (if InnerColorSchemeSelector_37 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . root . get () . unwrap () . upgrade () . unwrap () . globals . global_ColorSchemeSelector_37 . as_ref ()) . get () {
                                         sp :: Color :: from_argb_encoded (2147483648f64 as u32) }
                                     else {
                                         (sp :: Color :: from_argb_encoded (3019898879f64 as u32)) as _ }
                                    ) }
                                 else {
                                     (slint :: Brush :: SolidColor (if InnerColorSchemeSelector_37 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . root . get () . unwrap () . upgrade () . unwrap () . globals . global_ColorSchemeSelector_37 . as_ref ()) . get () {
                                         sp :: Color :: from_argb_encoded (3388997631f64 as u32) }
                                     else {
                                         (sp :: Color :: from_argb_encoded (2566914048f64 as u32)) as _ }
                                    )) as _ }
                                 }
                             else {
                                 (if ((r#tmp_root_25_state . clone () as f64) == (4f64 as f64)) {
                                     slint :: Brush :: SolidColor (if InnerColorSchemeSelector_37 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . root . get () . unwrap () . upgrade () . unwrap () . globals . global_ColorSchemeSelector_37 . as_ref ()) . get () {
                                         sp :: Color :: from_argb_encoded (4278190080f64 as u32) }
                                     else {
                                         (sp :: Color :: from_argb_encoded (4294967295f64 as u32)) as _ }
                                    ) }
                                 else {
                                     (if ((({
                                         * & InnerButton_root_25 :: FIELD_OFFSETS . r#root_25_primary }
                                    ) . apply_pin (_self) . get ()) || (({
                                         * & InnerButton_root_25 :: FIELD_OFFSETS . r#root_25_checked }
                                    ) . apply_pin (_self) . get ())) {
                                         slint :: Brush :: SolidColor (if InnerColorSchemeSelector_37 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . root . get () . unwrap () . upgrade () . unwrap () . globals . global_ColorSchemeSelector_37 . as_ref ()) . get () {
                                             sp :: Color :: from_argb_encoded (4278190080f64 as u32) }
                                         else {
                                             (sp :: Color :: from_argb_encoded (4294967295f64 as u32)) as _ }
                                        ) }
                                     else {
                                         (slint :: Brush :: SolidColor (if InnerColorSchemeSelector_37 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . root . get () . unwrap () . upgrade () . unwrap () . globals . global_ColorSchemeSelector_37 . as_ref ()) . get () {
                                             sp :: Color :: from_argb_encoded (4294967295f64 as u32) }
                                         else {
                                             (sp :: Color :: from_argb_encoded (3858759680f64 as u32)) as _ }
                                        )) as _ }
                                    ) as _ }
                                ) as _ }
                            ) as _ }
                         }
                    ) as _ }
                ) ;
                 }
             {
                 let _self = self_rc . as_pin_ref () ;
                 slint :: private_unstable_api :: set_animated_property_binding (({
                     * & InnerButton_root_25 :: FIELD_OFFSETS . r#i_background_26 }
                 + r#BorderRectangle :: FIELD_OFFSETS . r#background) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ({
                         let r#tmp_root_25_state = ({
                             * & InnerButton_root_25 :: FIELD_OFFSETS . r#root_25_state }
                        ) . apply_pin (_self) . get () ;
                         ;
                         if ((r#tmp_root_25_state . clone () as f64) == (1f64 as f64)) {
                             if ((({
                                 * & InnerButton_root_25 :: FIELD_OFFSETS . r#root_25_primary }
                            ) . apply_pin (_self) . get ()) || (({
                                 * & InnerButton_root_25 :: FIELD_OFFSETS . r#root_25_checked }
                            ) . apply_pin (_self) . get ())) {
                                 slint :: Brush :: SolidColor (if InnerColorSchemeSelector_37 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . root . get () . unwrap () . upgrade () . unwrap () . globals . global_ColorSchemeSelector_37 . as_ref ()) . get () {
                                     sp :: Color :: from_argb_encoded (704643071f64 as u32) }
                                 else {
                                     (sp :: Color :: from_argb_encoded (939524096f64 as u32)) as _ }
                                ) }
                             else {
                                 (slint :: Brush :: SolidColor (if InnerColorSchemeSelector_37 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . root . get () . unwrap () . upgrade () . unwrap () . globals . global_ColorSchemeSelector_37 . as_ref ()) . get () {
                                     sp :: Color :: from_argb_encoded (184549375f64 as u32) }
                                 else {
                                     (sp :: Color :: from_argb_encoded (1308228089f64 as u32)) as _ }
                                )) as _ }
                             }
                         else {
                             (if ((r#tmp_root_25_state . clone () as f64) == (2f64 as f64)) {
                                 if ((({
                                     * & InnerButton_root_25 :: FIELD_OFFSETS . r#root_25_primary }
                                ) . apply_pin (_self) . get ()) || (({
                                     * & InnerButton_root_25 :: FIELD_OFFSETS . r#root_25_checked }
                                ) . apply_pin (_self) . get ())) {
                                     slint :: Brush :: SolidColor (if InnerColorSchemeSelector_37 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . root . get () . unwrap () . upgrade () . unwrap () . globals . global_ColorSchemeSelector_37 . as_ref ()) . get () {
                                         sp :: Color :: from_argb_encoded (3428896255f64 as u32) }
                                     else {
                                         (sp :: Color :: from_argb_encoded (3422576568f64 as u32)) as _ }
                                    ) }
                                 else {
                                     (slint :: Brush :: SolidColor (if InnerColorSchemeSelector_37 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . root . get () . unwrap () . upgrade () . unwrap () . globals . global_ColorSchemeSelector_37 . as_ref ()) . get () {
                                         sp :: Color :: from_argb_encoded (150994943f64 as u32) }
                                     else {
                                         (sp :: Color :: from_argb_encoded (1308228089f64 as u32)) as _ }
                                    )) as _ }
                                 }
                             else {
                                 (if ((r#tmp_root_25_state . clone () as f64) == (3f64 as f64)) {
                                     if ((({
                                         * & InnerButton_root_25 :: FIELD_OFFSETS . r#root_25_primary }
                                    ) . apply_pin (_self) . get ()) || (({
                                         * & InnerButton_root_25 :: FIELD_OFFSETS . r#root_25_checked }
                                    ) . apply_pin (_self) . get ())) {
                                         slint :: Brush :: SolidColor (if InnerColorSchemeSelector_37 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . root . get () . unwrap () . upgrade () . unwrap () . globals . global_ColorSchemeSelector_37 . as_ref ()) . get () {
                                             sp :: Color :: from_argb_encoded (3865103871f64 as u32) }
                                         else {
                                             (sp :: Color :: from_argb_encoded (3858784184f64 as u32)) as _ }
                                        ) }
                                     else {
                                         (slint :: Brush :: SolidColor (if InnerColorSchemeSelector_37 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . root . get () . unwrap () . upgrade () . unwrap () . globals . global_ColorSchemeSelector_37 . as_ref ()) . get () {
                                             sp :: Color :: from_argb_encoded (352321535f64 as u32) }
                                         else {
                                             (sp :: Color :: from_argb_encoded (2163866105f64 as u32)) as _ }
                                        )) as _ }
                                     }
                                 else {
                                     (if ((r#tmp_root_25_state . clone () as f64) == (4f64 as f64)) {
                                         slint :: Brush :: SolidColor (if InnerColorSchemeSelector_37 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . root . get () . unwrap () . upgrade () . unwrap () . globals . global_ColorSchemeSelector_37 . as_ref ()) . get () {
                                             sp :: Color :: from_argb_encoded (4284534271f64 as u32) }
                                         else {
                                             (sp :: Color :: from_argb_encoded (4278214584f64 as u32)) as _ }
                                        ) }
                                     else {
                                         (if ({
                                             * & InnerButton_root_25 :: FIELD_OFFSETS . r#root_25_primary }
                                        ) . apply_pin (_self) . get () {
                                             slint :: Brush :: SolidColor (if InnerColorSchemeSelector_37 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . root . get () . unwrap () . upgrade () . unwrap () . globals . global_ColorSchemeSelector_37 . as_ref ()) . get () {
                                                 sp :: Color :: from_argb_encoded (4284534271f64 as u32) }
                                             else {
                                                 (sp :: Color :: from_argb_encoded (4278214584f64 as u32)) as _ }
                                            ) }
                                         else {
                                             (slint :: Brush :: SolidColor (if InnerColorSchemeSelector_37 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . root . get () . unwrap () . upgrade () . unwrap () . globals . global_ColorSchemeSelector_37 . as_ref ()) . get () {
                                                 sp :: Color :: from_argb_encoded (268435455f64 as u32) }
                                             else {
                                                 (sp :: Color :: from_argb_encoded (3019898879f64 as u32)) as _ }
                                            )) as _ }
                                        ) as _ }
                                    ) as _ }
                                ) as _ }
                            ) as _ }
                         }
                    ) as _ }
                 , {
                     let mut the_struct = r#PropertyAnimation :: default () ;
                     the_struct . r#delay = 0f64 as _ ;
                     the_struct . r#duration = 150f64 as _ ;
                     the_struct . r#easing = sp :: EasingCurve :: Linear as _ ;
                     the_struct . r#iteration_count = 1f64 as _ ;
                     the_struct }
                ) ;
                 }
             ({
                 * & InnerButton_root_25 :: FIELD_OFFSETS . r#i_background_26 }
             + r#BorderRectangle :: FIELD_OFFSETS . r#border_radius) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (4f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerButton_root_25 :: FIELD_OFFSETS . r#i_background_26 }
                 + r#BorderRectangle :: FIELD_OFFSETS . r#height) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (({
                         * & InnerButton_root_25 :: FIELD_OFFSETS . r#root_25 }
                     + r#Empty :: FIELD_OFFSETS . r#height) . apply_pin (_self) . get () . get () as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerButton_root_25 :: FIELD_OFFSETS . r#i_background_26 }
                 + r#BorderRectangle :: FIELD_OFFSETS . r#width) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (({
                         * & InnerButton_root_25 :: FIELD_OFFSETS . r#root_25 }
                     + r#Empty :: FIELD_OFFSETS . r#width) . apply_pin (_self) . get () . get () as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerButton_root_25 :: FIELD_OFFSETS . r#i_border_27 }
                 + r#BorderRectangle :: FIELD_OFFSETS . r#border_color) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ({
                         let r#tmp_root_25_state = ({
                             * & InnerButton_root_25 :: FIELD_OFFSETS . r#root_25_state }
                        ) . apply_pin (_self) . get () ;
                         ;
                         if ((r#tmp_root_25_state . clone () as f64) == (1f64 as f64)) {
                             if ((({
                                 * & InnerButton_root_25 :: FIELD_OFFSETS . r#root_25_primary }
                            ) . apply_pin (_self) . get ()) || (({
                                 * & InnerButton_root_25 :: FIELD_OFFSETS . r#root_25_checked }
                            ) . apply_pin (_self) . get ())) {
                                 slint :: Brush :: SolidColor (sp :: Color :: from_argb_encoded (0f64 as u32)) }
                             else {
                                 (slint :: Brush :: SolidColor (if InnerColorSchemeSelector_37 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . root . get () . unwrap () . upgrade () . unwrap () . globals . global_ColorSchemeSelector_37 . as_ref ()) . get () {
                                     sp :: Color :: from_argb_encoded (352321535f64 as u32) }
                                 else {
                                     (sp :: Color :: from_argb_encoded (251658240f64 as u32)) as _ }
                                )) as _ }
                             }
                         else {
                             (if ((r#tmp_root_25_state . clone () as f64) == (2f64 as f64)) {
                                 slint :: Brush :: SolidColor (if InnerColorSchemeSelector_37 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . root . get () . unwrap () . upgrade () . unwrap () . globals . global_ColorSchemeSelector_37 . as_ref ()) . get () {
                                     sp :: Color :: from_argb_encoded (352321535f64 as u32) }
                                 else {
                                     (sp :: Color :: from_argb_encoded (251658240f64 as u32)) as _ }
                                ) }
                             else {
                                 (if ((r#tmp_root_25_state . clone () as f64) == (4f64 as f64)) {
                                     InnerPalette_38 :: FIELD_OFFSETS . r#accent_control_border . apply_pin (_self . root . get () . unwrap () . upgrade () . unwrap () . globals . global_Palette_38 . as_ref ()) . get () }
                                 else {
                                     (if ({
                                         * & InnerButton_root_25 :: FIELD_OFFSETS . r#root_25_primary }
                                    ) . apply_pin (_self) . get () {
                                         InnerPalette_38 :: FIELD_OFFSETS . r#accent_control_border . apply_pin (_self . root . get () . unwrap () . upgrade () . unwrap () . globals . global_Palette_38 . as_ref ()) . get () }
                                     else {
                                         (InnerPalette_38 :: FIELD_OFFSETS . r#control_border . apply_pin (_self . root . get () . unwrap () . upgrade () . unwrap () . globals . global_Palette_38 . as_ref ()) . get ()) as _ }
                                    ) as _ }
                                ) as _ }
                            ) as _ }
                         }
                    ) as _ }
                ) ;
                 }
             ({
                 * & InnerButton_root_25 :: FIELD_OFFSETS . r#i_border_27 }
             + r#BorderRectangle :: FIELD_OFFSETS . r#border_radius) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (4f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 * & InnerButton_root_25 :: FIELD_OFFSETS . r#i_border_27 }
             + r#BorderRectangle :: FIELD_OFFSETS . r#border_width) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (1f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerButton_root_25 :: FIELD_OFFSETS . r#i_border_27 }
                 + r#BorderRectangle :: FIELD_OFFSETS . r#height) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (({
                         * & InnerButton_root_25 :: FIELD_OFFSETS . r#root_25 }
                     + r#Empty :: FIELD_OFFSETS . r#height) . apply_pin (_self) . get () . get () as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerButton_root_25 :: FIELD_OFFSETS . r#i_border_27 }
                 + r#BorderRectangle :: FIELD_OFFSETS . r#width) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (({
                         * & InnerButton_root_25 :: FIELD_OFFSETS . r#root_25 }
                     + r#Empty :: FIELD_OFFSETS . r#width) . apply_pin (_self) . get () . get () as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler (({
                     * & InnerButton_root_25 :: FIELD_OFFSETS . r#i_touch_area_33 }
                 + r#TouchArea :: FIELD_OFFSETS . r#clicked) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             if ({
                                 * & InnerButton_root_25 :: FIELD_OFFSETS . r#root_25_checkable }
                            ) . apply_pin (_self) . get () {
                                 {
                                     ({
                                         * & InnerButton_root_25 :: FIELD_OFFSETS . r#root_25_checked }
                                    ) . apply_pin (_self) . set (! ({
                                         * & InnerButton_root_25 :: FIELD_OFFSETS . r#root_25_checked }
                                    ) . apply_pin (_self) . get () as _) }
                                 }
                             else {
                                 ({
                                     }
                                ) as _ }
                             ;
                             ({
                                 * & InnerButton_root_25 :: FIELD_OFFSETS . r#root_25_clicked }
                            ) . apply_pin (_self) . call (& () . into ()) }
                        ) ;
                         }
                     }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerButton_root_25 :: FIELD_OFFSETS . r#i_touch_area_33 }
                 + r#TouchArea :: FIELD_OFFSETS . r#height) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (({
                         * & InnerButton_root_25 :: FIELD_OFFSETS . r#root_25 }
                     + r#Empty :: FIELD_OFFSETS . r#height) . apply_pin (_self) . get () . get () as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerButton_root_25 :: FIELD_OFFSETS . r#i_touch_area_33 }
                 + r#TouchArea :: FIELD_OFFSETS . r#width) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (({
                         * & InnerButton_root_25 :: FIELD_OFFSETS . r#root_25 }
                     + r#Empty :: FIELD_OFFSETS . r#width) . apply_pin (_self) . get () . get () as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerButton_root_25 :: FIELD_OFFSETS . r#i_focus_scope_34 }
                 + r#FocusScope :: FIELD_OFFSETS . r#enabled) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (true) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerButton_root_25 :: FIELD_OFFSETS . r#i_focus_scope_34 }
                 + r#FocusScope :: FIELD_OFFSETS . r#height) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (({
                         * & InnerButton_root_25 :: FIELD_OFFSETS . r#root_25 }
                     + r#Empty :: FIELD_OFFSETS . r#height) . apply_pin (_self) . get () . get () as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler (({
                     * & InnerButton_root_25 :: FIELD_OFFSETS . r#i_focus_scope_34 }
                 + r#FocusScope :: FIELD_OFFSETS . r#key_pressed) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             if (((((args . 0 . clone ()) . r#text) == (sp :: SharedString :: from (" ")))) || ((((args . 0 . clone ()) . r#text) == (sp :: SharedString :: from ("\n"))))) {
                                 {
                                     ({
                                         * & InnerButton_root_25 :: FIELD_OFFSETS . r#i_touch_area_33 }
                                     + r#TouchArea :: FIELD_OFFSETS . r#clicked) . apply_pin (_self) . call (& () . into ()) ;
                                     return (sp :: r#EventResult :: r#Accept) as _ ;
                                     }
                                 }
                             else {
                                 ({
                                     }
                                ) as _ }
                             ;
                             return (sp :: r#EventResult :: r#Reject) as _ ;
                             }
                        ) as _ }
                     }
                ) ;
                 }
             ({
                 * & InnerButton_root_25 :: FIELD_OFFSETS . r#i_focus_scope_34 }
             + r#FocusScope :: FIELD_OFFSETS . r#width) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (0f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 * & InnerButton_root_25 :: FIELD_OFFSETS . r#i_focus_scope_34 }
             + r#FocusScope :: FIELD_OFFSETS . r#x) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (0f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 * & InnerButton_root_25 :: FIELD_OFFSETS . r#root_25_checkable }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerButton_root_25 :: FIELD_OFFSETS . r#root_25_icon }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerButton_root_25 :: FIELD_OFFSETS . r#i_background_26 }
             + r#BorderRectangle :: FIELD_OFFSETS . r#border_color) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerButton_root_25 :: FIELD_OFFSETS . r#i_background_26 }
             + r#BorderRectangle :: FIELD_OFFSETS . r#border_radius) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerButton_root_25 :: FIELD_OFFSETS . r#i_background_26 }
             + r#BorderRectangle :: FIELD_OFFSETS . r#border_width) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerButton_root_25 :: FIELD_OFFSETS . r#i_background_26 }
             + r#BorderRectangle :: FIELD_OFFSETS . r#x) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerButton_root_25 :: FIELD_OFFSETS . r#i_background_26 }
             + r#BorderRectangle :: FIELD_OFFSETS . r#y) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerButton_root_25 :: FIELD_OFFSETS . r#i_border_27 }
             + r#BorderRectangle :: FIELD_OFFSETS . r#background) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerButton_root_25 :: FIELD_OFFSETS . r#i_border_27 }
             + r#BorderRectangle :: FIELD_OFFSETS . r#border_radius) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerButton_root_25 :: FIELD_OFFSETS . r#i_border_27 }
             + r#BorderRectangle :: FIELD_OFFSETS . r#border_width) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerButton_root_25 :: FIELD_OFFSETS . r#i_border_27 }
             + r#BorderRectangle :: FIELD_OFFSETS . r#x) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerButton_root_25 :: FIELD_OFFSETS . r#i_border_27 }
             + r#BorderRectangle :: FIELD_OFFSETS . r#y) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerButton_root_25 :: FIELD_OFFSETS . r#i_touch_area_33 }
             + r#TouchArea :: FIELD_OFFSETS . r#mouse_cursor) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerButton_root_25 :: FIELD_OFFSETS . r#i_touch_area_33 }
             + r#TouchArea :: FIELD_OFFSETS . r#x) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerButton_root_25 :: FIELD_OFFSETS . r#i_touch_area_33 }
             + r#TouchArea :: FIELD_OFFSETS . r#y) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerButton_root_25 :: FIELD_OFFSETS . r#i_focus_scope_34 }
             + r#FocusScope :: FIELD_OFFSETS . r#width) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerButton_root_25 :: FIELD_OFFSETS . r#i_focus_scope_34 }
             + r#FocusScope :: FIELD_OFFSETS . r#x) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerButton_root_25 :: FIELD_OFFSETS . r#i_focus_scope_34 }
             + r#FocusScope :: FIELD_OFFSETS . r#y) . apply_pin (_self) . set_constant () ;
             }
         pub fn user_init (self_rc : sp :: VRcMapped < sp :: ComponentVTable , Self >) {
             let _self = self_rc . as_pin_ref () ;
             }
         fn visit_dynamic_children (self : :: core :: pin :: Pin < & Self > , dyn_index : usize , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut) -> sp :: VisitChildrenResult {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 0usize => {
                     InnerButton_root_25 :: FIELD_OFFSETS . repeater0 . apply_pin (_self) . ensure_updated (|| InnerComponent_image_29 :: new (_self . self_weak . get () . unwrap () . clone ()) . into ()) ;
                     _self . repeater0 . visit (order , visitor) }
                 1usize => {
                     InnerButton_root_25 :: FIELD_OFFSETS . repeater1 . apply_pin (_self) . ensure_updated (|| InnerComponent_text_31 :: new (_self . self_weak . get () . unwrap () . clone ()) . into ()) ;
                     _self . repeater1 . visit (order , visitor) }
                 2usize => {
                     InnerButton_root_25 :: FIELD_OFFSETS . repeater2 . apply_pin (_self) . ensure_updated (|| InnerComponent_focusborder_35 :: new (_self . self_weak . get () . unwrap () . clone ()) . into ()) ;
                     _self . repeater2 . visit (order , visitor) }
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn layout_info (self : :: core :: pin :: Pin < & Self > , orientation : sp :: Orientation) -> sp :: LayoutInfo {
             # ! [allow (unused)] let _self = self ;
             match orientation {
                 sp :: Orientation :: Horizontal => {
                     let r#layout_info = (({
                         let mut the_struct = r#LayoutInfo :: default () ;
                         the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                         the_struct . r#max_percent = 100f64 as _ ;
                         the_struct . r#min = 0f64 as _ ;
                         the_struct . r#min_percent = 0f64 as _ ;
                         the_struct . r#preferred = 0f64 as _ ;
                         the_struct . r#stretch = 1f64 as _ ;
                         the_struct }
                    ) + ((({
                         let mut the_struct = r#LayoutInfo :: default () ;
                         the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                         the_struct . r#max_percent = 100f64 as _ ;
                         the_struct . r#min = 0f64 as _ ;
                         the_struct . r#min_percent = 0f64 as _ ;
                         the_struct . r#preferred = 0f64 as _ ;
                         the_struct . r#stretch = 1f64 as _ ;
                         the_struct }
                    ) + (({
                         * & InnerButton_root_25 :: FIELD_OFFSETS . r#root_25_i_layout_28_layoutinfo_h }
                    ) . apply_pin (_self) . get ())))) ;
                     ;
                     {
                         let mut the_struct = r#LayoutInfo :: default () ;
                         the_struct . r#max = (r#layout_info . clone ()) . r#max as _ ;
                         the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                         the_struct . r#min = (32f64 as sp :: Coord) . max (((({
                             * & InnerButton_root_25 :: FIELD_OFFSETS . r#root_25_i_layout_28_layoutinfo_h }
                        ) . apply_pin (_self) . get ()) . r#min as sp :: Coord)) as _ ;
                         the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                         the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                         the_struct . r#stretch = 0f64 as _ ;
                         the_struct }
                     }
                 , sp :: Orientation :: Vertical => {
                     let r#layout_info = (({
                         let mut the_struct = r#LayoutInfo :: default () ;
                         the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                         the_struct . r#max_percent = 100f64 as _ ;
                         the_struct . r#min = 0f64 as _ ;
                         the_struct . r#min_percent = 0f64 as _ ;
                         the_struct . r#preferred = 0f64 as _ ;
                         the_struct . r#stretch = 1f64 as _ ;
                         the_struct }
                    ) + ((({
                         let mut the_struct = r#LayoutInfo :: default () ;
                         the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                         the_struct . r#max_percent = 100f64 as _ ;
                         the_struct . r#min = 0f64 as _ ;
                         the_struct . r#min_percent = 0f64 as _ ;
                         the_struct . r#preferred = 0f64 as _ ;
                         the_struct . r#stretch = 1f64 as _ ;
                         the_struct }
                    ) + (({
                         * & InnerButton_root_25 :: FIELD_OFFSETS . r#root_25_i_layout_28_layoutinfo_v }
                    ) . apply_pin (_self) . get ())))) ;
                     ;
                     {
                         let mut the_struct = r#LayoutInfo :: default () ;
                         the_struct . r#max = (r#layout_info . clone ()) . r#max as _ ;
                         the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                         the_struct . r#min = (32f64 as sp :: Coord) . max (((({
                             * & InnerButton_root_25 :: FIELD_OFFSETS . r#root_25_i_layout_28_layoutinfo_v }
                        ) . apply_pin (_self) . get ()) . r#min as sp :: Coord)) as _ ;
                         the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                         the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                         the_struct . r#stretch = 0f64 as _ ;
                         the_struct }
                     }
                 , }
             }
         fn subtree_range (self : :: core :: pin :: Pin < & Self > , dyn_index : usize) -> sp :: IndexRange {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 0usize => {
                     InnerButton_root_25 :: FIELD_OFFSETS . repeater0 . apply_pin (_self) . ensure_updated (|| InnerComponent_image_29 :: new (_self . self_weak . get () . unwrap () . clone ()) . into ()) ;
                     sp :: IndexRange :: from (_self . repeater0 . range ()) }
                 1usize => {
                     InnerButton_root_25 :: FIELD_OFFSETS . repeater1 . apply_pin (_self) . ensure_updated (|| InnerComponent_text_31 :: new (_self . self_weak . get () . unwrap () . clone ()) . into ()) ;
                     sp :: IndexRange :: from (_self . repeater1 . range ()) }
                 2usize => {
                     InnerButton_root_25 :: FIELD_OFFSETS . repeater2 . apply_pin (_self) . ensure_updated (|| InnerComponent_focusborder_35 :: new (_self . self_weak . get () . unwrap () . clone ()) . into ()) ;
                     sp :: IndexRange :: from (_self . repeater2 . range ()) }
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn subtree_component (self : :: core :: pin :: Pin < & Self > , dyn_index : usize , subtree_index : usize , result : & mut sp :: ComponentWeak) {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 0usize => {
                     InnerButton_root_25 :: FIELD_OFFSETS . repeater0 . apply_pin (_self) . ensure_updated (|| InnerComponent_image_29 :: new (_self . self_weak . get () . unwrap () . clone ()) . into ()) ;
                     * result = vtable :: VRc :: downgrade (& vtable :: VRc :: into_dyn (_self . repeater0 . component_at (subtree_index) . unwrap ())) }
                 1usize => {
                     InnerButton_root_25 :: FIELD_OFFSETS . repeater1 . apply_pin (_self) . ensure_updated (|| InnerComponent_text_31 :: new (_self . self_weak . get () . unwrap () . clone ()) . into ()) ;
                     * result = vtable :: VRc :: downgrade (& vtable :: VRc :: into_dyn (_self . repeater1 . component_at (subtree_index) . unwrap ())) }
                 2usize => {
                     InnerButton_root_25 :: FIELD_OFFSETS . repeater2 . apply_pin (_self) . ensure_updated (|| InnerComponent_focusborder_35 :: new (_self . self_weak . get () . unwrap () . clone ()) . into ()) ;
                     * result = vtable :: VRc :: downgrade (& vtable :: VRc :: into_dyn (_self . repeater2 . component_at (subtree_index) . unwrap ())) }
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             ;
             }
         fn index_property (self : :: core :: pin :: Pin < & Self >) -> usize {
             # ! [allow (unused)] let _self = self ;
             core :: usize :: MAX }
         fn accessible_role (self : :: core :: pin :: Pin < & Self > , index : usize) -> sp :: AccessibleRole {
             # ! [allow (unused)] let _self = self ;
             match index {
                 0usize => sp :: r#AccessibleRole :: r#Button , _ => AccessibleRole :: default () , }
             }
         fn accessible_string_property (self : :: core :: pin :: Pin < & Self > , index : usize , what : sp :: AccessibleStringProperty ,) -> sp :: SharedString {
             # ! [allow (unused)] let _self = self ;
             match (index , what) {
                 (0usize , AccessibleStringProperty :: r#Label) => ({
                     * & InnerButton_root_25 :: FIELD_OFFSETS . r#root_25_text }
                ) . apply_pin (_self) . get () , _ => Default :: default () , }
             }
         }
     # [derive (sp :: FieldOffsets , Default)] # [const_field_offset (sp :: const_field_offset)] # [repr (C)] # [pin_drop] struct InnerComponent_image_29 {
         r#image_29 : sp :: r#ImageItem , self_weak : sp :: OnceCell < sp :: VWeakMapped < sp :: ComponentVTable , InnerComponent_image_29 >> , parent : sp :: VWeakMapped :: < sp :: ComponentVTable , InnerButton_root_25 > , root : sp :: OnceCell < sp :: VWeak < sp :: ComponentVTable , InnerMainWindow >> , tree_index : :: core :: cell :: Cell < u32 > , tree_index_of_first_child : :: core :: cell :: Cell < u32 > , }
     impl InnerComponent_image_29 {
         pub fn init (self_rc : sp :: VRcMapped < sp :: ComponentVTable , Self > , root : & sp :: VRc < sp :: ComponentVTable , InnerMainWindow > , tree_index : u32 , tree_index_of_first_child : u32) {
             # ! [allow (unused)] let _self = self_rc . as_pin_ref () ;
             _self . self_weak . set (VRcMapped :: downgrade (& self_rc)) ;
             _self . root . set (VRc :: downgrade (root)) ;
             _self . tree_index . set (tree_index) ;
             _self . tree_index_of_first_child . set (tree_index_of_first_child) ;
             Property :: link_two_way (({
                 * & InnerComponent_image_29 :: FIELD_OFFSETS . r#image_29 }
             + r#ImageItem :: FIELD_OFFSETS . r#source) . apply_pin (_self) , (InnerButton_root_25 :: FIELD_OFFSETS . r#root_25_icon) . apply_pin (_self . parent . upgrade () . unwrap () . as_pin_ref ())) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_image_29 :: FIELD_OFFSETS . r#image_29 }
                 + r#ImageItem :: FIELD_OFFSETS . r#height) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (((((({
                         * & InnerButton_root_25 :: FIELD_OFFSETS . r#root_25 }
                     + r#Empty :: FIELD_OFFSETS . r#height) . apply_pin (_self . parent . upgrade () . unwrap () . as_pin_ref ()) . get () . get () as f64) - (5f64 as f64)) as f64) - (5f64 as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & InnerComponent_image_29 :: FIELD_OFFSETS . r#image_29 }
             + r#ImageItem :: FIELD_OFFSETS . r#image_fit) . apply_pin (_self) . set ({
                 (sp :: r#ImageFit :: r#Contain) as sp :: r#ImageFit }
            ) ;
             ({
                 * & InnerComponent_image_29 :: FIELD_OFFSETS . r#image_29 }
             + r#ImageItem :: FIELD_OFFSETS . r#width) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (20f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_image_29 :: FIELD_OFFSETS . r#image_29 }
                 + r#ImageItem :: FIELD_OFFSETS . r#x) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ({
                         let cache = (InnerButton_root_25 :: FIELD_OFFSETS . r#root_25_i_layout_28_layout_cache) . apply_pin (_self . parent . upgrade () . unwrap () . as_pin_ref ()) . get () ;
                         * cache . get ((cache [0usize] as usize) + 0f64 as usize * 2) . unwrap_or (& (0 as sp :: Coord)) }
                     as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & InnerComponent_image_29 :: FIELD_OFFSETS . r#image_29 }
             + r#ImageItem :: FIELD_OFFSETS . r#y) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (5f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 * & InnerComponent_image_29 :: FIELD_OFFSETS . r#image_29 }
             + r#ImageItem :: FIELD_OFFSETS . r#colorize) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_image_29 :: FIELD_OFFSETS . r#image_29 }
             + r#ImageItem :: FIELD_OFFSETS . r#image_fit) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_image_29 :: FIELD_OFFSETS . r#image_29 }
             + r#ImageItem :: FIELD_OFFSETS . r#image_rendering) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_image_29 :: FIELD_OFFSETS . r#image_29 }
             + r#ImageItem :: FIELD_OFFSETS . r#source) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_image_29 :: FIELD_OFFSETS . r#image_29 }
             + r#ImageItem :: FIELD_OFFSETS . r#width) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_image_29 :: FIELD_OFFSETS . r#image_29 }
             + r#ImageItem :: FIELD_OFFSETS . r#y) . apply_pin (_self) . set_constant () ;
             }
         pub fn user_init (self_rc : sp :: VRcMapped < sp :: ComponentVTable , Self >) {
             let _self = self_rc . as_pin_ref () ;
             }
         fn visit_dynamic_children (self : :: core :: pin :: Pin < & Self > , dyn_index : usize , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut) -> sp :: VisitChildrenResult {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn layout_info (self : :: core :: pin :: Pin < & Self > , orientation : sp :: Orientation) -> sp :: LayoutInfo {
             # ! [allow (unused)] let _self = self ;
             match orientation {
                 sp :: Orientation :: Horizontal => {
                     let r#layout_info = sp :: Item :: layout_info (({
                         * & InnerComponent_image_29 :: FIELD_OFFSETS . r#image_29 }
                    ) . apply_pin (_self) , sp :: Orientation :: Horizontal , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ())) ;
                     ;
                     {
                         let mut the_struct = r#LayoutInfo :: default () ;
                         the_struct . r#max = 20f64 as _ ;
                         the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                         the_struct . r#min = 20f64 as _ ;
                         the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                         the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                         the_struct . r#stretch = (r#layout_info . clone ()) . r#stretch as _ ;
                         the_struct }
                     }
                 , sp :: Orientation :: Vertical => {
                     let r#layout_info = sp :: Item :: layout_info (({
                         * & InnerComponent_image_29 :: FIELD_OFFSETS . r#image_29 }
                    ) . apply_pin (_self) , sp :: Orientation :: Vertical , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ())) ;
                     ;
                     {
                         let mut the_struct = r#LayoutInfo :: default () ;
                         the_struct . r#max = (r#layout_info . clone ()) . r#max as _ ;
                         the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                         the_struct . r#min = (r#layout_info . clone ()) . r#min as _ ;
                         the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                         the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                         the_struct . r#stretch = (r#layout_info . clone ()) . r#stretch as _ ;
                         the_struct }
                     }
                 , }
             }
         fn subtree_range (self : :: core :: pin :: Pin < & Self > , dyn_index : usize) -> sp :: IndexRange {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn subtree_component (self : :: core :: pin :: Pin < & Self > , dyn_index : usize , subtree_index : usize , result : & mut sp :: ComponentWeak) {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             ;
             }
         fn index_property (self : :: core :: pin :: Pin < & Self >) -> usize {
             # ! [allow (unused)] let _self = self ;
             core :: usize :: MAX }
         fn accessible_role (self : :: core :: pin :: Pin < & Self > , index : usize) -> sp :: AccessibleRole {
             # ! [allow (unused)] let _self = self ;
             match index {
                 _ => AccessibleRole :: default () , }
             }
         fn accessible_string_property (self : :: core :: pin :: Pin < & Self > , index : usize , what : sp :: AccessibleStringProperty ,) -> sp :: SharedString {
             # ! [allow (unused)] let _self = self ;
             match (index , what) {
                 _ => Default :: default () , }
             }
         }
     impl InnerComponent_image_29 {
         pub fn new (parent : sp :: VWeakMapped :: < sp :: ComponentVTable , InnerButton_root_25 >) -> vtable :: VRc < sp :: ComponentVTable , Self > {
             # ! [allow (unused)] slint :: private_unstable_api :: ensure_backend () ;
             let mut _self = Self :: default () ;
             _self . parent = parent . clone () as sp :: VWeakMapped :: < sp :: ComponentVTable , InnerButton_root_25 > ;
             let self_rc = VRc :: new (_self) ;
             let self_dyn_rc = vtable :: VRc :: into_dyn (self_rc . clone ()) ;
             sp :: register_component (& self_dyn_rc , (* & parent . upgrade () . unwrap () . root . get () . unwrap () . upgrade () . unwrap ()) . maybe_window_adapter_impl ()) ;
             Self :: init (sp :: VRc :: map (self_rc . clone () , | x | x) , & parent . upgrade () . unwrap () . root . get () . unwrap () . upgrade () . unwrap () , 0 , 1) ;
             self_rc }
         fn item_tree () -> & 'static [sp :: ItemTreeNode] {
             const ITEM_TREE : [sp :: ItemTreeNode ;
             1usize] = [sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 1u32 , parent_index : 0u32 , item_array_index : 0u32 , }
            ] ;
             & ITEM_TREE }
         fn item_array () -> & 'static [vtable :: VOffset < Self , ItemVTable , vtable :: AllowPin >] {
             static ITEM_ARRAY : sp :: OnceBox < [vtable :: VOffset < InnerComponent_image_29 , ItemVTable , vtable :: AllowPin > ;
             1usize] > = sp :: OnceBox :: new () ;
             & * ITEM_ARRAY . get_or_init (|| Box :: new ([VOffset :: new ({
                 * & InnerComponent_image_29 :: FIELD_OFFSETS . r#image_29 }
            )])) }
         # [allow (unused)] fn window_adapter_impl (& self) -> Rc < dyn sp :: WindowAdapter > {
             self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl () }
         # [allow (unused)] fn maybe_window_adapter_impl (& self) -> Option < Rc < dyn sp :: WindowAdapter >> {
             self . root . get () . and_then (| root_weak | root_weak . upgrade ()) . and_then (| root | root . maybe_window_adapter_impl ()) }
         }
     impl sp :: PinnedDrop for InnerComponent_image_29 {
         fn drop (self : core :: pin :: Pin < & mut InnerComponent_image_29 >) {
             use slint :: private_unstable_api :: re_exports :: * ;
             ComponentVTable_static ! (static VT for self :: InnerComponent_image_29) ;
             new_vref ! (let vref : VRef < ComponentVTable > for Component = self . as_ref () . get_ref ()) ;
             if let Some (wa) = self . maybe_window_adapter_impl () {
                 sp :: unregister_component (self . as_ref () , vref , Self :: item_array () , & wa) ;
                 }
             }
         }
     impl sp :: Component for InnerComponent_image_29 {
         fn visit_children_item (self : :: core :: pin :: Pin < & Self > , index : isize , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut) -> sp :: VisitChildrenResult {
             return sp :: visit_item_tree (self , & VRcMapped :: origin (& self . as_ref () . self_weak . get () . unwrap () . upgrade () . unwrap ()) , self . get_item_tree () . as_slice () , index , order , visitor , visit_dynamic) ;
             # [allow (unused)] fn visit_dynamic (_self : :: core :: pin :: Pin < & InnerComponent_image_29 > , order : sp :: TraversalOrder , visitor : ItemVisitorRefMut , dyn_index : usize) -> VisitChildrenResult {
                 _self . visit_dynamic_children (dyn_index , order , visitor) }
             }
         fn get_item_ref (self : :: core :: pin :: Pin < & Self > , index : usize) -> :: core :: pin :: Pin < ItemRef > {
             match & self . get_item_tree () . as_slice () [index] {
                 ItemTreeNode :: Item {
                     item_array_index , .. }
                 => {
                     Self :: item_array () [* item_array_index as usize] . apply_pin (self) }
                 ItemTreeNode :: DynamicTree {
                     .. }
                 => panic ! ("get_item_ref called on dynamic tree") , }
             }
         fn get_item_tree (self : :: core :: pin :: Pin < & Self >) -> sp :: Slice < sp :: ItemTreeNode > {
             Self :: item_tree () . into () }
         fn get_subtree_range (self : :: core :: pin :: Pin < & Self > , index : usize) -> sp :: IndexRange {
             self . subtree_range (index) }
         fn get_subtree_component (self : :: core :: pin :: Pin < & Self > , index : usize , subtree_index : usize , result : & mut sp :: ComponentWeak) {
             self . subtree_component (index , subtree_index , result) ;
             }
         fn subtree_index (self : :: core :: pin :: Pin < & Self >) -> usize {
             self . index_property () }
         fn parent_node (self : :: core :: pin :: Pin < & Self > , _result : & mut sp :: ItemWeak) {
             if let Some ((parent_component , parent_index)) = self . parent . clone () . upgrade () . map (| sc | (VRcMapped :: origin (& sc) , sc . tree_index_of_first_child . get ())) {
                 * _result = sp :: ItemRc :: new (parent_component , parent_index as usize + 6usize - 1) . downgrade () ;
                 }
             }
         fn embed_component (self : :: core :: pin :: Pin < & Self > , _parent_component : & sp :: ComponentWeak , _item_tree_index : usize) -> bool {
             todo ! ("Components written in Rust can not get embedded yet.") }
         fn layout_info (self : :: core :: pin :: Pin < & Self > , orientation : sp :: Orientation) -> sp :: LayoutInfo {
             self . layout_info (orientation) }
         fn accessible_role (self : :: core :: pin :: Pin < & Self > , index : usize) -> sp :: AccessibleRole {
             self . accessible_role (index) }
         fn accessible_string_property (self : :: core :: pin :: Pin < & Self > , index : usize , what : sp :: AccessibleStringProperty , result : & mut sp :: SharedString ,) {
             * result = self . accessible_string_property (index , what) ;
             }
         fn window_adapter (self : :: core :: pin :: Pin < & Self > , do_create : bool , result : & mut Option < Rc < dyn WindowAdapter >> ,) {
             if do_create {
                 * result = Some (self . window_adapter_impl ()) ;
                 }
             else {
                 * result = self . maybe_window_adapter_impl () ;
                 }
             }
         }
     impl sp :: RepeatedComponent for InnerComponent_image_29 {
         type Data = () ;
         fn update (& self , _index : usize , _data : Self :: Data) {
             let self_rc = self . self_weak . get () . unwrap () . upgrade () . unwrap () ;
             let _self = self_rc . as_pin_ref () ;
             }
         fn init (& self) {
             let self_rc = self . self_weak . get () . unwrap () . upgrade () . unwrap () ;
             InnerComponent_image_29 :: user_init (VRcMapped :: map (self_rc , | x | x) ,) ;
             }
         fn box_layout_data (self : :: core :: pin :: Pin < & Self > , o : sp :: Orientation) -> sp :: BoxLayoutCellData {
             BoxLayoutCellData {
                 constraint : self . as_ref () . layout_info (o) }
             }
         }
     # [derive (sp :: FieldOffsets , Default)] # [const_field_offset (sp :: const_field_offset)] # [repr (C)] # [pin_drop] struct InnerComponent_text_31 {
         r#text_31 : sp :: r#Text , self_weak : sp :: OnceCell < sp :: VWeakMapped < sp :: ComponentVTable , InnerComponent_text_31 >> , parent : sp :: VWeakMapped :: < sp :: ComponentVTable , InnerButton_root_25 > , root : sp :: OnceCell < sp :: VWeak < sp :: ComponentVTable , InnerMainWindow >> , tree_index : :: core :: cell :: Cell < u32 > , tree_index_of_first_child : :: core :: cell :: Cell < u32 > , }
     impl InnerComponent_text_31 {
         pub fn init (self_rc : sp :: VRcMapped < sp :: ComponentVTable , Self > , root : & sp :: VRc < sp :: ComponentVTable , InnerMainWindow > , tree_index : u32 , tree_index_of_first_child : u32) {
             # ! [allow (unused)] let _self = self_rc . as_pin_ref () ;
             _self . self_weak . set (VRcMapped :: downgrade (& self_rc)) ;
             _self . root . set (VRc :: downgrade (root)) ;
             _self . tree_index . set (tree_index) ;
             _self . tree_index_of_first_child . set (tree_index_of_first_child) ;
             {
                 let _self = self_rc . as_pin_ref () ;
                 slint :: private_unstable_api :: set_animated_property_binding (({
                     * & InnerComponent_text_31 :: FIELD_OFFSETS . r#text_31 }
                 + r#Text :: FIELD_OFFSETS . r#color) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ((InnerButton_root_25 :: FIELD_OFFSETS . r#root_25_text_color) . apply_pin (_self . parent . upgrade () . unwrap () . as_pin_ref ()) . get ()) as _ }
                 , {
                     let mut the_struct = r#PropertyAnimation :: default () ;
                     the_struct . r#delay = 0f64 as _ ;
                     the_struct . r#duration = 150f64 as _ ;
                     the_struct . r#easing = sp :: EasingCurve :: Linear as _ ;
                     the_struct . r#iteration_count = 1f64 as _ ;
                     the_struct }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_text_31 :: FIELD_OFFSETS . r#text_31 }
                 + r#Text :: FIELD_OFFSETS . r#font_size) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (((({
                         let obj = (1.0766f64 as f32 , 400f64 as i32 ,) ;
                         let mut the_struct = r#TextStyle :: default () ;
                         the_struct . r#font_size = obj . 0 as _ ;
                         the_struct . r#font_weight = obj . 1 as _ ;
                         the_struct }
                    ) . r#font_size as f64) * (({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#root_1 }
                     + sp :: WindowItem :: FIELD_OFFSETS . default_font_size) . apply_pin (_self . root . get () . unwrap () . upgrade () . unwrap () . as_pin_ref ()) . get () . get () as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & InnerComponent_text_31 :: FIELD_OFFSETS . r#text_31 }
             + r#Text :: FIELD_OFFSETS . r#font_weight) . apply_pin (_self) . set ({
                 (({
                     let obj = (1.0766f64 as f32 , 400f64 as i32 ,) ;
                     let mut the_struct = r#TextStyle :: default () ;
                     the_struct . r#font_size = obj . 0 as _ ;
                     the_struct . r#font_weight = obj . 1 as _ ;
                     the_struct }
                ) . r#font_weight) as i32 }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_text_31 :: FIELD_OFFSETS . r#text_31 }
                 + r#Text :: FIELD_OFFSETS . r#height) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (((((({
                         * & InnerButton_root_25 :: FIELD_OFFSETS . r#root_25 }
                     + r#Empty :: FIELD_OFFSETS . r#height) . apply_pin (_self . parent . upgrade () . unwrap () . as_pin_ref ()) . get () . get () as f64) - (5f64 as f64)) as f64) - (5f64 as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & InnerComponent_text_31 :: FIELD_OFFSETS . r#text_31 }
             + r#Text :: FIELD_OFFSETS . r#horizontal_alignment) . apply_pin (_self) . set ({
                 (sp :: r#TextHorizontalAlignment :: r#Center) as sp :: r#TextHorizontalAlignment }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_text_31 :: FIELD_OFFSETS . r#text_31 }
                 + r#Text :: FIELD_OFFSETS . r#text) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ((InnerButton_root_25 :: FIELD_OFFSETS . r#root_25_text) . apply_pin (_self . parent . upgrade () . unwrap () . as_pin_ref ()) . get ()) as _ }
                ) ;
                 }
             ({
                 * & InnerComponent_text_31 :: FIELD_OFFSETS . r#text_31 }
             + r#Text :: FIELD_OFFSETS . r#vertical_alignment) . apply_pin (_self) . set ({
                 (sp :: r#TextVerticalAlignment :: r#Center) as sp :: r#TextVerticalAlignment }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_text_31 :: FIELD_OFFSETS . r#text_31 }
                 + r#Text :: FIELD_OFFSETS . r#width) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ({
                         let cache = (InnerButton_root_25 :: FIELD_OFFSETS . r#root_25_i_layout_28_layout_cache) . apply_pin (_self . parent . upgrade () . unwrap () . as_pin_ref ()) . get () ;
                         * cache . get ((cache [3usize] as usize) + 0f64 as usize * 2) . unwrap_or (& (0 as sp :: Coord)) }
                     as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_text_31 :: FIELD_OFFSETS . r#text_31 }
                 + r#Text :: FIELD_OFFSETS . r#x) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ({
                         let cache = (InnerButton_root_25 :: FIELD_OFFSETS . r#root_25_i_layout_28_layout_cache) . apply_pin (_self . parent . upgrade () . unwrap () . as_pin_ref ()) . get () ;
                         * cache . get ((cache [2usize] as usize) + 0f64 as usize * 2) . unwrap_or (& (0 as sp :: Coord)) }
                     as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & InnerComponent_text_31 :: FIELD_OFFSETS . r#text_31 }
             + r#Text :: FIELD_OFFSETS . r#y) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (5f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 * & InnerComponent_text_31 :: FIELD_OFFSETS . r#text_31 }
             + r#Text :: FIELD_OFFSETS . r#font_family) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_text_31 :: FIELD_OFFSETS . r#text_31 }
             + r#Text :: FIELD_OFFSETS . r#font_italic) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_text_31 :: FIELD_OFFSETS . r#text_31 }
             + r#Text :: FIELD_OFFSETS . r#font_weight) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_text_31 :: FIELD_OFFSETS . r#text_31 }
             + r#Text :: FIELD_OFFSETS . r#horizontal_alignment) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_text_31 :: FIELD_OFFSETS . r#text_31 }
             + r#Text :: FIELD_OFFSETS . r#letter_spacing) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_text_31 :: FIELD_OFFSETS . r#text_31 }
             + r#Text :: FIELD_OFFSETS . r#overflow) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_text_31 :: FIELD_OFFSETS . r#text_31 }
             + r#Text :: FIELD_OFFSETS . r#vertical_alignment) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_text_31 :: FIELD_OFFSETS . r#text_31 }
             + r#Text :: FIELD_OFFSETS . r#wrap) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_text_31 :: FIELD_OFFSETS . r#text_31 }
             + r#Text :: FIELD_OFFSETS . r#y) . apply_pin (_self) . set_constant () ;
             }
         pub fn user_init (self_rc : sp :: VRcMapped < sp :: ComponentVTable , Self >) {
             let _self = self_rc . as_pin_ref () ;
             }
         fn visit_dynamic_children (self : :: core :: pin :: Pin < & Self > , dyn_index : usize , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut) -> sp :: VisitChildrenResult {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn layout_info (self : :: core :: pin :: Pin < & Self > , orientation : sp :: Orientation) -> sp :: LayoutInfo {
             # ! [allow (unused)] let _self = self ;
             match orientation {
                 sp :: Orientation :: Horizontal => sp :: Item :: layout_info (({
                     * & InnerComponent_text_31 :: FIELD_OFFSETS . r#text_31 }
                ) . apply_pin (_self) , sp :: Orientation :: Horizontal , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ())) , sp :: Orientation :: Vertical => sp :: Item :: layout_info (({
                     * & InnerComponent_text_31 :: FIELD_OFFSETS . r#text_31 }
                ) . apply_pin (_self) , sp :: Orientation :: Vertical , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ())) , }
             }
         fn subtree_range (self : :: core :: pin :: Pin < & Self > , dyn_index : usize) -> sp :: IndexRange {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn subtree_component (self : :: core :: pin :: Pin < & Self > , dyn_index : usize , subtree_index : usize , result : & mut sp :: ComponentWeak) {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             ;
             }
         fn index_property (self : :: core :: pin :: Pin < & Self >) -> usize {
             # ! [allow (unused)] let _self = self ;
             core :: usize :: MAX }
         fn accessible_role (self : :: core :: pin :: Pin < & Self > , index : usize) -> sp :: AccessibleRole {
             # ! [allow (unused)] let _self = self ;
             match index {
                 0usize => sp :: r#AccessibleRole :: r#Text , _ => AccessibleRole :: default () , }
             }
         fn accessible_string_property (self : :: core :: pin :: Pin < & Self > , index : usize , what : sp :: AccessibleStringProperty ,) -> sp :: SharedString {
             # ! [allow (unused)] let _self = self ;
             match (index , what) {
                 (0usize , AccessibleStringProperty :: r#Label) => (InnerButton_root_25 :: FIELD_OFFSETS . r#root_25_text) . apply_pin (_self . parent . upgrade () . unwrap () . as_pin_ref ()) . get () , _ => Default :: default () , }
             }
         }
     impl InnerComponent_text_31 {
         pub fn new (parent : sp :: VWeakMapped :: < sp :: ComponentVTable , InnerButton_root_25 >) -> vtable :: VRc < sp :: ComponentVTable , Self > {
             # ! [allow (unused)] slint :: private_unstable_api :: ensure_backend () ;
             let mut _self = Self :: default () ;
             _self . parent = parent . clone () as sp :: VWeakMapped :: < sp :: ComponentVTable , InnerButton_root_25 > ;
             let self_rc = VRc :: new (_self) ;
             let self_dyn_rc = vtable :: VRc :: into_dyn (self_rc . clone ()) ;
             sp :: register_component (& self_dyn_rc , (* & parent . upgrade () . unwrap () . root . get () . unwrap () . upgrade () . unwrap ()) . maybe_window_adapter_impl ()) ;
             Self :: init (sp :: VRc :: map (self_rc . clone () , | x | x) , & parent . upgrade () . unwrap () . root . get () . unwrap () . upgrade () . unwrap () , 0 , 1) ;
             self_rc }
         fn item_tree () -> & 'static [sp :: ItemTreeNode] {
             const ITEM_TREE : [sp :: ItemTreeNode ;
             1usize] = [sp :: ItemTreeNode :: Item {
                 is_accessible : true , children_count : 0u32 , children_index : 1u32 , parent_index : 0u32 , item_array_index : 0u32 , }
            ] ;
             & ITEM_TREE }
         fn item_array () -> & 'static [vtable :: VOffset < Self , ItemVTable , vtable :: AllowPin >] {
             static ITEM_ARRAY : sp :: OnceBox < [vtable :: VOffset < InnerComponent_text_31 , ItemVTable , vtable :: AllowPin > ;
             1usize] > = sp :: OnceBox :: new () ;
             & * ITEM_ARRAY . get_or_init (|| Box :: new ([VOffset :: new ({
                 * & InnerComponent_text_31 :: FIELD_OFFSETS . r#text_31 }
            )])) }
         # [allow (unused)] fn window_adapter_impl (& self) -> Rc < dyn sp :: WindowAdapter > {
             self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl () }
         # [allow (unused)] fn maybe_window_adapter_impl (& self) -> Option < Rc < dyn sp :: WindowAdapter >> {
             self . root . get () . and_then (| root_weak | root_weak . upgrade ()) . and_then (| root | root . maybe_window_adapter_impl ()) }
         }
     impl sp :: PinnedDrop for InnerComponent_text_31 {
         fn drop (self : core :: pin :: Pin < & mut InnerComponent_text_31 >) {
             use slint :: private_unstable_api :: re_exports :: * ;
             ComponentVTable_static ! (static VT for self :: InnerComponent_text_31) ;
             new_vref ! (let vref : VRef < ComponentVTable > for Component = self . as_ref () . get_ref ()) ;
             if let Some (wa) = self . maybe_window_adapter_impl () {
                 sp :: unregister_component (self . as_ref () , vref , Self :: item_array () , & wa) ;
                 }
             }
         }
     impl sp :: Component for InnerComponent_text_31 {
         fn visit_children_item (self : :: core :: pin :: Pin < & Self > , index : isize , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut) -> sp :: VisitChildrenResult {
             return sp :: visit_item_tree (self , & VRcMapped :: origin (& self . as_ref () . self_weak . get () . unwrap () . upgrade () . unwrap ()) , self . get_item_tree () . as_slice () , index , order , visitor , visit_dynamic) ;
             # [allow (unused)] fn visit_dynamic (_self : :: core :: pin :: Pin < & InnerComponent_text_31 > , order : sp :: TraversalOrder , visitor : ItemVisitorRefMut , dyn_index : usize) -> VisitChildrenResult {
                 _self . visit_dynamic_children (dyn_index , order , visitor) }
             }
         fn get_item_ref (self : :: core :: pin :: Pin < & Self > , index : usize) -> :: core :: pin :: Pin < ItemRef > {
             match & self . get_item_tree () . as_slice () [index] {
                 ItemTreeNode :: Item {
                     item_array_index , .. }
                 => {
                     Self :: item_array () [* item_array_index as usize] . apply_pin (self) }
                 ItemTreeNode :: DynamicTree {
                     .. }
                 => panic ! ("get_item_ref called on dynamic tree") , }
             }
         fn get_item_tree (self : :: core :: pin :: Pin < & Self >) -> sp :: Slice < sp :: ItemTreeNode > {
             Self :: item_tree () . into () }
         fn get_subtree_range (self : :: core :: pin :: Pin < & Self > , index : usize) -> sp :: IndexRange {
             self . subtree_range (index) }
         fn get_subtree_component (self : :: core :: pin :: Pin < & Self > , index : usize , subtree_index : usize , result : & mut sp :: ComponentWeak) {
             self . subtree_component (index , subtree_index , result) ;
             }
         fn subtree_index (self : :: core :: pin :: Pin < & Self >) -> usize {
             self . index_property () }
         fn parent_node (self : :: core :: pin :: Pin < & Self > , _result : & mut sp :: ItemWeak) {
             if let Some ((parent_component , parent_index)) = self . parent . clone () . upgrade () . map (| sc | (VRcMapped :: origin (& sc) , sc . tree_index_of_first_child . get ())) {
                 * _result = sp :: ItemRc :: new (parent_component , parent_index as usize + 7usize - 1) . downgrade () ;
                 }
             }
         fn embed_component (self : :: core :: pin :: Pin < & Self > , _parent_component : & sp :: ComponentWeak , _item_tree_index : usize) -> bool {
             todo ! ("Components written in Rust can not get embedded yet.") }
         fn layout_info (self : :: core :: pin :: Pin < & Self > , orientation : sp :: Orientation) -> sp :: LayoutInfo {
             self . layout_info (orientation) }
         fn accessible_role (self : :: core :: pin :: Pin < & Self > , index : usize) -> sp :: AccessibleRole {
             self . accessible_role (index) }
         fn accessible_string_property (self : :: core :: pin :: Pin < & Self > , index : usize , what : sp :: AccessibleStringProperty , result : & mut sp :: SharedString ,) {
             * result = self . accessible_string_property (index , what) ;
             }
         fn window_adapter (self : :: core :: pin :: Pin < & Self > , do_create : bool , result : & mut Option < Rc < dyn WindowAdapter >> ,) {
             if do_create {
                 * result = Some (self . window_adapter_impl ()) ;
                 }
             else {
                 * result = self . maybe_window_adapter_impl () ;
                 }
             }
         }
     impl sp :: RepeatedComponent for InnerComponent_text_31 {
         type Data = () ;
         fn update (& self , _index : usize , _data : Self :: Data) {
             let self_rc = self . self_weak . get () . unwrap () . upgrade () . unwrap () ;
             let _self = self_rc . as_pin_ref () ;
             }
         fn init (& self) {
             let self_rc = self . self_weak . get () . unwrap () . upgrade () . unwrap () ;
             InnerComponent_text_31 :: user_init (VRcMapped :: map (self_rc , | x | x) ,) ;
             }
         fn box_layout_data (self : :: core :: pin :: Pin < & Self > , o : sp :: Orientation) -> sp :: BoxLayoutCellData {
             BoxLayoutCellData {
                 constraint : self . as_ref () . layout_info (o) }
             }
         }
     # [derive (sp :: FieldOffsets , Default)] # [const_field_offset (sp :: const_field_offset)] # [repr (C)] # [pin_drop] struct InnerComponent_focusborder_35 {
         r#focusborder_35 : InnerFocusBorder_root_23 , self_weak : sp :: OnceCell < sp :: VWeakMapped < sp :: ComponentVTable , InnerComponent_focusborder_35 >> , parent : sp :: VWeakMapped :: < sp :: ComponentVTable , InnerButton_root_25 > , root : sp :: OnceCell < sp :: VWeak < sp :: ComponentVTable , InnerMainWindow >> , tree_index : :: core :: cell :: Cell < u32 > , tree_index_of_first_child : :: core :: cell :: Cell < u32 > , }
     impl InnerComponent_focusborder_35 {
         pub fn init (self_rc : sp :: VRcMapped < sp :: ComponentVTable , Self > , root : & sp :: VRc < sp :: ComponentVTable , InnerMainWindow > , tree_index : u32 , tree_index_of_first_child : u32) {
             # ! [allow (unused)] let _self = self_rc . as_pin_ref () ;
             _self . self_weak . set (VRcMapped :: downgrade (& self_rc)) ;
             _self . root . set (VRc :: downgrade (root)) ;
             _self . tree_index . set (tree_index) ;
             _self . tree_index_of_first_child . set (tree_index_of_first_child) ;
             InnerFocusBorder_root_23 :: init (VRcMapped :: map (self_rc . clone () , | x | {
                 * & Self :: FIELD_OFFSETS . r#focusborder_35 }
             . apply_pin (x)) , & _self . root . get () . unwrap () . upgrade () . unwrap () , tree_index , tree_index_of_first_child + 1u32 - 1) ;
             ({
                 InnerComponent_focusborder_35 :: FIELD_OFFSETS . r#focusborder_35 }
             + {
                 * & InnerFocusBorder_root_23 :: FIELD_OFFSETS . r#root_23 }
             + r#BorderRectangle :: FIELD_OFFSETS . r#border_radius) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (4f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerComponent_focusborder_35 :: FIELD_OFFSETS . r#focusborder_35 }
                 + {
                     * & InnerFocusBorder_root_23 :: FIELD_OFFSETS . r#root_23 }
                 + r#BorderRectangle :: FIELD_OFFSETS . r#height) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (({
                         * & InnerButton_root_25 :: FIELD_OFFSETS . r#root_25 }
                     + r#Empty :: FIELD_OFFSETS . r#height) . apply_pin (_self . parent . upgrade () . unwrap () . as_pin_ref ()) . get () . get () as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerComponent_focusborder_35 :: FIELD_OFFSETS . r#focusborder_35 }
                 + {
                     * & InnerFocusBorder_root_23 :: FIELD_OFFSETS . r#root_23 }
                 + r#BorderRectangle :: FIELD_OFFSETS . r#width) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (({
                         * & InnerButton_root_25 :: FIELD_OFFSETS . r#root_25 }
                     + r#Empty :: FIELD_OFFSETS . r#width) . apply_pin (_self . parent . upgrade () . unwrap () . as_pin_ref ()) . get () . get () as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 InnerComponent_focusborder_35 :: FIELD_OFFSETS . r#focusborder_35 }
             + {
                 * & InnerFocusBorder_root_23 :: FIELD_OFFSETS . r#root_23 }
             + r#BorderRectangle :: FIELD_OFFSETS . r#background) . apply_pin (_self) . set_constant () ;
             ({
                 InnerComponent_focusborder_35 :: FIELD_OFFSETS . r#focusborder_35 }
             + {
                 * & InnerFocusBorder_root_23 :: FIELD_OFFSETS . r#root_23 }
             + r#BorderRectangle :: FIELD_OFFSETS . r#border_radius) . apply_pin (_self) . set_constant () ;
             ({
                 InnerComponent_focusborder_35 :: FIELD_OFFSETS . r#focusborder_35 }
             + {
                 * & InnerFocusBorder_root_23 :: FIELD_OFFSETS . r#root_23 }
             + r#BorderRectangle :: FIELD_OFFSETS . r#border_width) . apply_pin (_self) . set_constant () ;
             ({
                 InnerComponent_focusborder_35 :: FIELD_OFFSETS . r#focusborder_35 }
             + {
                 * & InnerFocusBorder_root_23 :: FIELD_OFFSETS . r#root_23 }
             + r#BorderRectangle :: FIELD_OFFSETS . r#x) . apply_pin (_self) . set_constant () ;
             ({
                 InnerComponent_focusborder_35 :: FIELD_OFFSETS . r#focusborder_35 }
             + {
                 * & InnerFocusBorder_root_23 :: FIELD_OFFSETS . r#root_23 }
             + r#BorderRectangle :: FIELD_OFFSETS . r#y) . apply_pin (_self) . set_constant () ;
             }
         pub fn user_init (self_rc : sp :: VRcMapped < sp :: ComponentVTable , Self >) {
             let _self = self_rc . as_pin_ref () ;
             InnerFocusBorder_root_23 :: user_init (VRcMapped :: map (self_rc . clone () , | x | {
                 * & Self :: FIELD_OFFSETS . r#focusborder_35 }
             . apply_pin (x)) ,) ;
             }
         fn visit_dynamic_children (self : :: core :: pin :: Pin < & Self > , dyn_index : usize , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut) -> sp :: VisitChildrenResult {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn layout_info (self : :: core :: pin :: Pin < & Self > , orientation : sp :: Orientation) -> sp :: LayoutInfo {
             # ! [allow (unused)] let _self = self ;
             match orientation {
                 sp :: Orientation :: Horizontal => sp :: Item :: layout_info (({
                     InnerComponent_focusborder_35 :: FIELD_OFFSETS . r#focusborder_35 }
                 + {
                     * & InnerFocusBorder_root_23 :: FIELD_OFFSETS . r#root_23 }
                ) . apply_pin (_self) , sp :: Orientation :: Horizontal , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ())) , sp :: Orientation :: Vertical => sp :: Item :: layout_info (({
                     InnerComponent_focusborder_35 :: FIELD_OFFSETS . r#focusborder_35 }
                 + {
                     * & InnerFocusBorder_root_23 :: FIELD_OFFSETS . r#root_23 }
                ) . apply_pin (_self) , sp :: Orientation :: Vertical , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ())) , }
             }
         fn subtree_range (self : :: core :: pin :: Pin < & Self > , dyn_index : usize) -> sp :: IndexRange {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn subtree_component (self : :: core :: pin :: Pin < & Self > , dyn_index : usize , subtree_index : usize , result : & mut sp :: ComponentWeak) {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             ;
             }
         fn index_property (self : :: core :: pin :: Pin < & Self >) -> usize {
             # ! [allow (unused)] let _self = self ;
             core :: usize :: MAX }
         fn accessible_role (self : :: core :: pin :: Pin < & Self > , index : usize) -> sp :: AccessibleRole {
             # ! [allow (unused)] let _self = self ;
             match index {
                 0usize => {
                     * & Self :: FIELD_OFFSETS . r#focusborder_35 }
                 . apply_pin (_self) . accessible_role (0) , 1usize ..= 1usize => {
                     * & Self :: FIELD_OFFSETS . r#focusborder_35 }
                 . apply_pin (_self) . accessible_role (index - 1usize + 1) , _ => AccessibleRole :: default () , }
             }
         fn accessible_string_property (self : :: core :: pin :: Pin < & Self > , index : usize , what : sp :: AccessibleStringProperty ,) -> sp :: SharedString {
             # ! [allow (unused)] let _self = self ;
             match (index , what) {
                 (0usize , _) => {
                     * & Self :: FIELD_OFFSETS . r#focusborder_35 }
                 . apply_pin (_self) . accessible_string_property (0 , what) , (1usize ..= 1usize , _) => {
                     * & Self :: FIELD_OFFSETS . r#focusborder_35 }
                 . apply_pin (_self) . accessible_string_property (index - 1usize + 1 , what) , _ => Default :: default () , }
             }
         }
     impl InnerComponent_focusborder_35 {
         pub fn new (parent : sp :: VWeakMapped :: < sp :: ComponentVTable , InnerButton_root_25 >) -> vtable :: VRc < sp :: ComponentVTable , Self > {
             # ! [allow (unused)] slint :: private_unstable_api :: ensure_backend () ;
             let mut _self = Self :: default () ;
             _self . parent = parent . clone () as sp :: VWeakMapped :: < sp :: ComponentVTable , InnerButton_root_25 > ;
             let self_rc = VRc :: new (_self) ;
             let self_dyn_rc = vtable :: VRc :: into_dyn (self_rc . clone ()) ;
             sp :: register_component (& self_dyn_rc , (* & parent . upgrade () . unwrap () . root . get () . unwrap () . upgrade () . unwrap ()) . maybe_window_adapter_impl ()) ;
             Self :: init (sp :: VRc :: map (self_rc . clone () , | x | x) , & parent . upgrade () . unwrap () . root . get () . unwrap () . upgrade () . unwrap () , 0 , 1) ;
             self_rc }
         fn item_tree () -> & 'static [sp :: ItemTreeNode] {
             const ITEM_TREE : [sp :: ItemTreeNode ;
             2usize] = [sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 1u32 , parent_index : 0u32 , item_array_index : 0u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 2u32 , parent_index : 0u32 , item_array_index : 1u32 , }
            ] ;
             & ITEM_TREE }
         fn item_array () -> & 'static [vtable :: VOffset < Self , ItemVTable , vtable :: AllowPin >] {
             static ITEM_ARRAY : sp :: OnceBox < [vtable :: VOffset < InnerComponent_focusborder_35 , ItemVTable , vtable :: AllowPin > ;
             2usize] > = sp :: OnceBox :: new () ;
             & * ITEM_ARRAY . get_or_init (|| Box :: new ([VOffset :: new ({
                 InnerComponent_focusborder_35 :: FIELD_OFFSETS . r#focusborder_35 }
             + {
                 * & InnerFocusBorder_root_23 :: FIELD_OFFSETS . r#root_23 }
            ) , VOffset :: new ({
                 InnerComponent_focusborder_35 :: FIELD_OFFSETS . r#focusborder_35 }
             + {
                 * & InnerFocusBorder_root_23 :: FIELD_OFFSETS . r#rectangle_24 }
            )])) }
         # [allow (unused)] fn window_adapter_impl (& self) -> Rc < dyn sp :: WindowAdapter > {
             self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl () }
         # [allow (unused)] fn maybe_window_adapter_impl (& self) -> Option < Rc < dyn sp :: WindowAdapter >> {
             self . root . get () . and_then (| root_weak | root_weak . upgrade ()) . and_then (| root | root . maybe_window_adapter_impl ()) }
         }
     impl sp :: PinnedDrop for InnerComponent_focusborder_35 {
         fn drop (self : core :: pin :: Pin < & mut InnerComponent_focusborder_35 >) {
             use slint :: private_unstable_api :: re_exports :: * ;
             ComponentVTable_static ! (static VT for self :: InnerComponent_focusborder_35) ;
             new_vref ! (let vref : VRef < ComponentVTable > for Component = self . as_ref () . get_ref ()) ;
             if let Some (wa) = self . maybe_window_adapter_impl () {
                 sp :: unregister_component (self . as_ref () , vref , Self :: item_array () , & wa) ;
                 }
             }
         }
     impl sp :: Component for InnerComponent_focusborder_35 {
         fn visit_children_item (self : :: core :: pin :: Pin < & Self > , index : isize , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut) -> sp :: VisitChildrenResult {
             return sp :: visit_item_tree (self , & VRcMapped :: origin (& self . as_ref () . self_weak . get () . unwrap () . upgrade () . unwrap ()) , self . get_item_tree () . as_slice () , index , order , visitor , visit_dynamic) ;
             # [allow (unused)] fn visit_dynamic (_self : :: core :: pin :: Pin < & InnerComponent_focusborder_35 > , order : sp :: TraversalOrder , visitor : ItemVisitorRefMut , dyn_index : usize) -> VisitChildrenResult {
                 _self . visit_dynamic_children (dyn_index , order , visitor) }
             }
         fn get_item_ref (self : :: core :: pin :: Pin < & Self > , index : usize) -> :: core :: pin :: Pin < ItemRef > {
             match & self . get_item_tree () . as_slice () [index] {
                 ItemTreeNode :: Item {
                     item_array_index , .. }
                 => {
                     Self :: item_array () [* item_array_index as usize] . apply_pin (self) }
                 ItemTreeNode :: DynamicTree {
                     .. }
                 => panic ! ("get_item_ref called on dynamic tree") , }
             }
         fn get_item_tree (self : :: core :: pin :: Pin < & Self >) -> sp :: Slice < sp :: ItemTreeNode > {
             Self :: item_tree () . into () }
         fn get_subtree_range (self : :: core :: pin :: Pin < & Self > , index : usize) -> sp :: IndexRange {
             self . subtree_range (index) }
         fn get_subtree_component (self : :: core :: pin :: Pin < & Self > , index : usize , subtree_index : usize , result : & mut sp :: ComponentWeak) {
             self . subtree_component (index , subtree_index , result) ;
             }
         fn subtree_index (self : :: core :: pin :: Pin < & Self >) -> usize {
             self . index_property () }
         fn parent_node (self : :: core :: pin :: Pin < & Self > , _result : & mut sp :: ItemWeak) {
             if let Some ((parent_component , parent_index)) = self . parent . clone () . upgrade () . map (| sc | (VRcMapped :: origin (& sc) , sc . tree_index_of_first_child . get ())) {
                 * _result = sp :: ItemRc :: new (parent_component , parent_index as usize + 4usize - 1) . downgrade () ;
                 }
             }
         fn embed_component (self : :: core :: pin :: Pin < & Self > , _parent_component : & sp :: ComponentWeak , _item_tree_index : usize) -> bool {
             todo ! ("Components written in Rust can not get embedded yet.") }
         fn layout_info (self : :: core :: pin :: Pin < & Self > , orientation : sp :: Orientation) -> sp :: LayoutInfo {
             self . layout_info (orientation) }
         fn accessible_role (self : :: core :: pin :: Pin < & Self > , index : usize) -> sp :: AccessibleRole {
             self . accessible_role (index) }
         fn accessible_string_property (self : :: core :: pin :: Pin < & Self > , index : usize , what : sp :: AccessibleStringProperty , result : & mut sp :: SharedString ,) {
             * result = self . accessible_string_property (index , what) ;
             }
         fn window_adapter (self : :: core :: pin :: Pin < & Self > , do_create : bool , result : & mut Option < Rc < dyn WindowAdapter >> ,) {
             if do_create {
                 * result = Some (self . window_adapter_impl ()) ;
                 }
             else {
                 * result = self . maybe_window_adapter_impl () ;
                 }
             }
         }
     impl sp :: RepeatedComponent for InnerComponent_focusborder_35 {
         type Data = () ;
         fn update (& self , _index : usize , _data : Self :: Data) {
             let self_rc = self . self_weak . get () . unwrap () . upgrade () . unwrap () ;
             let _self = self_rc . as_pin_ref () ;
             }
         fn init (& self) {
             let self_rc = self . self_weak . get () . unwrap () . upgrade () . unwrap () ;
             InnerComponent_focusborder_35 :: user_init (VRcMapped :: map (self_rc , | x | x) ,) ;
             }
         fn box_layout_data (self : :: core :: pin :: Pin < & Self > , o : sp :: Orientation) -> sp :: BoxLayoutCellData {
             BoxLayoutCellData {
                 constraint : self . as_ref () . layout_info (o) }
             }
         }
     # [derive (sp :: FieldOffsets , Default)] # [const_field_offset (sp :: const_field_offset)] # [repr (C)] # [pin_drop] pub struct InnerMainWindow {
         r#root_1 : sp :: r#WindowItem , r#empty_3 : sp :: r#Empty , r#text_4 : sp :: r#Text , r#text_5 : sp :: r#Text , r#_shadow_6 : sp :: r#BoxShadow , r#rectangle_7 : sp :: r#BorderRectangle , r#text_9 : sp :: r#Text , r#text_10 : sp :: r#Text , r#auth_input_11 : sp :: r#Empty , r#i_background_12 : sp :: r#BorderRectangle , r#rectangle_14 : sp :: r#Empty , r#_clip_15 : sp :: r#Clip , r#i_placeholder_visibility_16 : sp :: r#Clip , r#i_placeholder_17 : sp :: r#Text , r#i_text_input_18 : sp :: r#TextInput , r#i_focus_border_19 : sp :: r#Rectangle , r#button_22 : InnerButton_root_25 , r#root_1_auth_input_11_state : sp :: Property < i32 > , r#root_1_connection_status : sp :: Property < sp :: SharedString > , r#root_1_empty_2_layout_cache : sp :: Property < sp :: SharedVector < sp :: Coord , > > , r#root_1_empty_2_layoutinfo_h : sp :: Property < r#LayoutInfo > , r#root_1_empty_2_layoutinfo_v : sp :: Property < r#LayoutInfo > , r#root_1_empty_3_layout_cache : sp :: Property < sp :: SharedVector < sp :: Coord , > > , r#root_1_empty_3_layoutinfo_h : sp :: Property < r#LayoutInfo > , r#root_1_empty_3_layoutinfo_v : sp :: Property < r#LayoutInfo > , r#root_1_empty_8_layout_cache : sp :: Property < sp :: SharedVector < sp :: Coord , > > , r#root_1_empty_8_layoutinfo_h : sp :: Property < r#LayoutInfo > , r#root_1_empty_8_layoutinfo_v : sp :: Property < r#LayoutInfo > , r#root_1_error_message : sp :: Property < sp :: SharedString > , r#root_1_i_layout_13_layout_cache : sp :: Property < sp :: SharedVector < sp :: Coord , > > , r#root_1_i_layout_13_layoutinfo_h : sp :: Property < r#LayoutInfo > , r#root_1_i_layout_13_layoutinfo_v : sp :: Property < r#LayoutInfo > , r#root_1_i_placeholder_17_horizontal_stretch : sp :: Property < f32 > , r#root_1_i_placeholder_17_max_height : sp :: Property < sp :: LogicalLength > , r#root_1_i_placeholder_17_max_width : sp :: Property < sp :: LogicalLength > , r#root_1_i_placeholder_17_min_height : sp :: Property < sp :: LogicalLength > , r#root_1_i_placeholder_17_min_width : sp :: Property < sp :: LogicalLength > , r#root_1_i_placeholder_17_preferred_height : sp :: Property < sp :: LogicalLength > , r#root_1_i_placeholder_17_preferred_width : sp :: Property < sp :: LogicalLength > , r#root_1_i_placeholder_17_vertical_stretch : sp :: Property < f32 > , r#root_1_i_placeholder_17_visible : sp :: Property < bool > , r#root_1_i_text_input_18_computed_x : sp :: Property < sp :: LogicalLength > , r#root_1_i_text_input_18_preferred_width : sp :: Property < sp :: LogicalLength > , r#root_1_is_connected : sp :: Property < bool > , r#root_1_layoutinfo_h : sp :: Property < r#LayoutInfo > , r#root_1_layoutinfo_v : sp :: Property < r#LayoutInfo > , r#root_1_rectangle_14_layoutinfo_h : sp :: Property < r#LayoutInfo > , r#root_1_rectangle_14_layoutinfo_v : sp :: Property < r#LayoutInfo > , r#root_1_auth_input_11_accepted : sp :: Callback < (sp :: SharedString ,) , () > , r#root_1_auth_input_11_edited : sp :: Callback < (sp :: SharedString ,) , () > , r#root_1_connect_clicked : sp :: Callback < (sp :: SharedString ,) , () > , repeater0 : sp :: Repeater < InnerComponent_text_20 > , self_weak : sp :: OnceCell < sp :: VWeakMapped < sp :: ComponentVTable , InnerMainWindow >> , root : sp :: OnceCell < sp :: VWeak < sp :: ComponentVTable , InnerMainWindow >> , tree_index : :: core :: cell :: Cell < u32 > , tree_index_of_first_child : :: core :: cell :: Cell < u32 > , globals : Globals_MainWindow , window_adapter_ : sp :: OnceCell < sp :: WindowAdapterRc > , }
     impl InnerMainWindow {
         pub fn init (self_rc : sp :: VRcMapped < sp :: ComponentVTable , Self > , root : & sp :: VRc < sp :: ComponentVTable , InnerMainWindow > , tree_index : u32 , tree_index_of_first_child : u32) {
             # ! [allow (unused)] let _self = self_rc . as_pin_ref () ;
             _self . self_weak . set (VRcMapped :: downgrade (& self_rc)) ;
             _self . root . set (VRc :: downgrade (root)) ;
             _self . tree_index . set (tree_index) ;
             _self . tree_index_of_first_child . set (tree_index_of_first_child) ;
             _self . repeater0 . set_model_binding ({
                 let self_weak = sp :: VRcMapped :: downgrade (& self_rc) ;
                 move || {
                     let self_rc = self_weak . upgrade () . unwrap () ;
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: ModelRc :: new (((({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_error_message }
                    ) . apply_pin (_self) . get ()) != (sp :: SharedString :: from (""))) as bool)) as _ }
                 }
            ) ;
             InnerButton_root_25 :: init (VRcMapped :: map (self_rc . clone () , | x | {
                 * & Self :: FIELD_OFFSETS . r#button_22 }
             . apply_pin (x)) , & _self . root . get () . unwrap () . upgrade () . unwrap () , tree_index_of_first_child + 6u32 - 1 , tree_index_of_first_child + 18u32 - 1) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_auth_input_11_state }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (if ! ({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#i_text_input_18 }
                     + r#TextInput :: FIELD_OFFSETS . r#enabled) . apply_pin (_self) . get () {
                         1f64 }
                     else {
                         (if ({
                             * & InnerMainWindow :: FIELD_OFFSETS . r#i_text_input_18 }
                         + r#TextInput :: FIELD_OFFSETS . r#has_focus) . apply_pin (_self) . get () {
                             2f64 }
                         else {
                             (0f64) as _ }
                        ) as _ }
                    ) as _ }
                ) ;
                 }
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#root_1 }
             + r#WindowItem :: FIELD_OFFSETS . r#background) . apply_pin (_self) . set ({
                 (slint :: Brush :: SolidColor (sp :: Color :: from_argb_encoded (4279179050f64 as u32))) as slint :: Brush }
            ) ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_connection_status }
            ) . apply_pin (_self) . set ({
                 (sp :: SharedString :: from ("离线")) as sp :: SharedString }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_empty_2_layout_cache }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ({
                         let mut r#repeated_indices = [0u32 ;
                         2 * 1usize] ;
                         let mut items_vec = sp :: Vec :: with_capacity (4usize + _self . repeater0 . len ()) ;
                         items_vec . push ({
                             let mut the_struct = r#BoxLayoutCellData :: default () ;
                             the_struct . r#constraint = ({
                                 * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_empty_3_layoutinfo_v }
                            ) . apply_pin (_self) . get () as _ ;
                             the_struct }
                        ) ;
                         items_vec . push ({
                             let mut the_struct = r#BoxLayoutCellData :: default () ;
                             the_struct . r#constraint = {
                                 let r#layout_info = (({
                                     let mut the_struct = r#LayoutInfo :: default () ;
                                     the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                                     the_struct . r#max_percent = 100f64 as _ ;
                                     the_struct . r#min = 0f64 as _ ;
                                     the_struct . r#min_percent = 0f64 as _ ;
                                     the_struct . r#preferred = 0f64 as _ ;
                                     the_struct . r#stretch = 1f64 as _ ;
                                     the_struct }
                                ) + (({
                                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_empty_8_layoutinfo_v }
                                ) . apply_pin (_self) . get ())) ;
                                 ;
                                 {
                                     let mut the_struct = r#LayoutInfo :: default () ;
                                     the_struct . r#max = 80f64 as _ ;
                                     the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                     the_struct . r#min = 80f64 as _ ;
                                     the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                     the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                     the_struct . r#stretch = (r#layout_info . clone ()) . r#stretch as _ ;
                                     the_struct }
                                 }
                             as _ ;
                             the_struct }
                        ) ;
                         items_vec . push ({
                             let mut the_struct = r#BoxLayoutCellData :: default () ;
                             the_struct . r#constraint = {
                                 let r#layout_info = (({
                                     let mut the_struct = r#LayoutInfo :: default () ;
                                     the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                                     the_struct . r#max_percent = 100f64 as _ ;
                                     the_struct . r#min = 0f64 as _ ;
                                     the_struct . r#min_percent = 0f64 as _ ;
                                     the_struct . r#preferred = 0f64 as _ ;
                                     the_struct . r#stretch = 1f64 as _ ;
                                     the_struct }
                                ) + ((({
                                     let mut the_struct = r#LayoutInfo :: default () ;
                                     the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                                     the_struct . r#max_percent = 100f64 as _ ;
                                     the_struct . r#min = 0f64 as _ ;
                                     the_struct . r#min_percent = 0f64 as _ ;
                                     the_struct . r#preferred = 0f64 as _ ;
                                     the_struct . r#stretch = 1f64 as _ ;
                                     the_struct }
                                ) + (({
                                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_i_layout_13_layoutinfo_v }
                                ) . apply_pin (_self) . get ())))) ;
                                 ;
                                 {
                                     let mut the_struct = r#LayoutInfo :: default () ;
                                     the_struct . r#max = 45f64 as _ ;
                                     the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                     the_struct . r#min = 45f64 as _ ;
                                     the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                     the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                     the_struct . r#stretch = 0f64 as _ ;
                                     the_struct }
                                 }
                             as _ ;
                             the_struct }
                        ) ;
                         InnerMainWindow :: FIELD_OFFSETS . repeater0 . apply_pin (_self) . ensure_updated (|| {
                             InnerComponent_text_20 :: new (_self . self_weak . get () . unwrap () . clone ()) . into () }
                        ) ;
                         let internal_vec = _self . repeater0 . components_vec () ;
                         r#repeated_indices [0usize * 2] = items_vec . len () as u32 ;
                         r#repeated_indices [0usize * 2 + 1] = internal_vec . len () as u32 ;
                         for sub_comp in & internal_vec {
                             items_vec . push (sub_comp . as_pin_ref () . box_layout_data (sp :: Orientation :: Vertical)) }
                         items_vec . push ({
                             let mut the_struct = r#BoxLayoutCellData :: default () ;
                             the_struct . r#constraint = {
                                 let r#layout_info = (({
                                     let mut the_struct = r#LayoutInfo :: default () ;
                                     the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                                     the_struct . r#max_percent = 100f64 as _ ;
                                     the_struct . r#min = 0f64 as _ ;
                                     the_struct . r#min_percent = 0f64 as _ ;
                                     the_struct . r#preferred = 0f64 as _ ;
                                     the_struct . r#stretch = 1f64 as _ ;
                                     the_struct }
                                ) + ((({
                                     let mut the_struct = r#LayoutInfo :: default () ;
                                     the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                                     the_struct . r#max_percent = 100f64 as _ ;
                                     the_struct . r#min = 0f64 as _ ;
                                     the_struct . r#min_percent = 0f64 as _ ;
                                     the_struct . r#preferred = 0f64 as _ ;
                                     the_struct . r#stretch = 1f64 as _ ;
                                     the_struct }
                                ) + (({
                                     InnerMainWindow :: FIELD_OFFSETS . r#button_22 }
                                 + {
                                     * & InnerButton_root_25 :: FIELD_OFFSETS . r#root_25_i_layout_28_layoutinfo_v }
                                ) . apply_pin (_self) . get ())))) ;
                                 ;
                                 {
                                     let mut the_struct = r#LayoutInfo :: default () ;
                                     the_struct . r#max = 45f64 as _ ;
                                     the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                     the_struct . r#min = 45f64 as _ ;
                                     the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                     the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                     the_struct . r#stretch = 0f64 as _ ;
                                     the_struct }
                                 }
                             as _ ;
                             the_struct }
                        ) ;
                         let r#cells = sp :: Slice :: from_slice (& items_vec) ;
                         let r#repeated_indices = sp :: Slice :: from_slice (& r#repeated_indices) ;
                         r#solve_box_layout (& r#BoxLayoutData {
                             r#alignment : sp :: r#LayoutAlignment :: r#Start as _ , r#cells : r#cells . clone () as _ , r#padding : {
                                 let mut the_struct = r#Padding :: default () ;
                                 the_struct . r#begin = 40f64 as _ ;
                                 the_struct . r#end = 40f64 as _ ;
                                 the_struct }
                             as _ , r#size : ({
                                 * & InnerMainWindow :: FIELD_OFFSETS . r#root_1 }
                             + r#WindowItem :: FIELD_OFFSETS . r#height) . apply_pin (_self) . get () . get () as _ , r#spacing : 24f64 as _ , }
                         as _ , r#repeated_indices . clone () as _) }
                    ) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_empty_2_layoutinfo_h }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ({
                         let mut items_vec = sp :: Vec :: with_capacity (4usize + _self . repeater0 . len ()) ;
                         items_vec . push ({
                             let mut the_struct = r#BoxLayoutCellData :: default () ;
                             the_struct . r#constraint = ({
                                 * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_empty_3_layoutinfo_h }
                            ) . apply_pin (_self) . get () as _ ;
                             the_struct }
                        ) ;
                         items_vec . push ({
                             let mut the_struct = r#BoxLayoutCellData :: default () ;
                             the_struct . r#constraint = {
                                 let r#layout_info = (({
                                     let mut the_struct = r#LayoutInfo :: default () ;
                                     the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                                     the_struct . r#max_percent = 100f64 as _ ;
                                     the_struct . r#min = 0f64 as _ ;
                                     the_struct . r#min_percent = 0f64 as _ ;
                                     the_struct . r#preferred = 0f64 as _ ;
                                     the_struct . r#stretch = 1f64 as _ ;
                                     the_struct }
                                ) + (({
                                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_empty_8_layoutinfo_h }
                                ) . apply_pin (_self) . get ())) ;
                                 ;
                                 {
                                     let mut the_struct = r#LayoutInfo :: default () ;
                                     the_struct . r#max = (r#layout_info . clone ()) . r#max as _ ;
                                     the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                     the_struct . r#min = (r#layout_info . clone ()) . r#min as _ ;
                                     the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                     the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                     the_struct . r#stretch = (r#layout_info . clone ()) . r#stretch as _ ;
                                     the_struct }
                                 }
                             as _ ;
                             the_struct }
                        ) ;
                         items_vec . push ({
                             let mut the_struct = r#BoxLayoutCellData :: default () ;
                             the_struct . r#constraint = {
                                 let r#layout_info = (({
                                     let mut the_struct = r#LayoutInfo :: default () ;
                                     the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                                     the_struct . r#max_percent = 100f64 as _ ;
                                     the_struct . r#min = 0f64 as _ ;
                                     the_struct . r#min_percent = 0f64 as _ ;
                                     the_struct . r#preferred = 0f64 as _ ;
                                     the_struct . r#stretch = 1f64 as _ ;
                                     the_struct }
                                ) + ((({
                                     let mut the_struct = r#LayoutInfo :: default () ;
                                     the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                                     the_struct . r#max_percent = 100f64 as _ ;
                                     the_struct . r#min = 0f64 as _ ;
                                     the_struct . r#min_percent = 0f64 as _ ;
                                     the_struct . r#preferred = 0f64 as _ ;
                                     the_struct . r#stretch = 1f64 as _ ;
                                     the_struct }
                                ) + (({
                                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_i_layout_13_layoutinfo_h }
                                ) . apply_pin (_self) . get ())))) ;
                                 ;
                                 {
                                     let mut the_struct = r#LayoutInfo :: default () ;
                                     the_struct . r#max = (r#layout_info . clone ()) . r#max as _ ;
                                     the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                     the_struct . r#min = (160f64 as sp :: Coord) . max (((({
                                         * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_i_layout_13_layoutinfo_v }
                                    ) . apply_pin (_self) . get ()) . r#min as sp :: Coord)) as _ ;
                                     the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                     the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                     the_struct . r#stretch = 1f64 as _ ;
                                     the_struct }
                                 }
                             as _ ;
                             the_struct }
                        ) ;
                         InnerMainWindow :: FIELD_OFFSETS . repeater0 . apply_pin (_self) . ensure_updated (|| {
                             InnerComponent_text_20 :: new (_self . self_weak . get () . unwrap () . clone ()) . into () }
                        ) ;
                         let internal_vec = _self . repeater0 . components_vec () ;
                         for sub_comp in & internal_vec {
                             items_vec . push (sub_comp . as_pin_ref () . box_layout_data (sp :: Orientation :: Horizontal)) }
                         items_vec . push ({
                             let mut the_struct = r#BoxLayoutCellData :: default () ;
                             the_struct . r#constraint = {
                                 let r#layout_info = (({
                                     let mut the_struct = r#LayoutInfo :: default () ;
                                     the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                                     the_struct . r#max_percent = 100f64 as _ ;
                                     the_struct . r#min = 0f64 as _ ;
                                     the_struct . r#min_percent = 0f64 as _ ;
                                     the_struct . r#preferred = 0f64 as _ ;
                                     the_struct . r#stretch = 1f64 as _ ;
                                     the_struct }
                                ) + ((({
                                     let mut the_struct = r#LayoutInfo :: default () ;
                                     the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                                     the_struct . r#max_percent = 100f64 as _ ;
                                     the_struct . r#min = 0f64 as _ ;
                                     the_struct . r#min_percent = 0f64 as _ ;
                                     the_struct . r#preferred = 0f64 as _ ;
                                     the_struct . r#stretch = 1f64 as _ ;
                                     the_struct }
                                ) + (({
                                     InnerMainWindow :: FIELD_OFFSETS . r#button_22 }
                                 + {
                                     * & InnerButton_root_25 :: FIELD_OFFSETS . r#root_25_i_layout_28_layoutinfo_h }
                                ) . apply_pin (_self) . get ())))) ;
                                 ;
                                 {
                                     let mut the_struct = r#LayoutInfo :: default () ;
                                     the_struct . r#max = (r#layout_info . clone ()) . r#max as _ ;
                                     the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                     the_struct . r#min = (32f64 as sp :: Coord) . max (((({
                                         InnerMainWindow :: FIELD_OFFSETS . r#button_22 }
                                     + {
                                         * & InnerButton_root_25 :: FIELD_OFFSETS . r#root_25_i_layout_28_layoutinfo_h }
                                    ) . apply_pin (_self) . get ()) . r#min as sp :: Coord)) as _ ;
                                     the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                     the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                     the_struct . r#stretch = 0f64 as _ ;
                                     the_struct }
                                 }
                             as _ ;
                             the_struct }
                        ) ;
                         let r#cells = sp :: Slice :: from_slice (& items_vec) ;
                         r#box_layout_info_ortho (r#cells . clone () as _ , & {
                             let mut the_struct = r#Padding :: default () ;
                             the_struct . r#begin = 40f64 as _ ;
                             the_struct . r#end = 40f64 as _ ;
                             the_struct }
                         as _) }
                    ) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_empty_2_layoutinfo_v }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ({
                         let mut items_vec = sp :: Vec :: with_capacity (4usize + _self . repeater0 . len ()) ;
                         items_vec . push ({
                             let mut the_struct = r#BoxLayoutCellData :: default () ;
                             the_struct . r#constraint = ({
                                 * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_empty_3_layoutinfo_v }
                            ) . apply_pin (_self) . get () as _ ;
                             the_struct }
                        ) ;
                         items_vec . push ({
                             let mut the_struct = r#BoxLayoutCellData :: default () ;
                             the_struct . r#constraint = {
                                 let r#layout_info = (({
                                     let mut the_struct = r#LayoutInfo :: default () ;
                                     the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                                     the_struct . r#max_percent = 100f64 as _ ;
                                     the_struct . r#min = 0f64 as _ ;
                                     the_struct . r#min_percent = 0f64 as _ ;
                                     the_struct . r#preferred = 0f64 as _ ;
                                     the_struct . r#stretch = 1f64 as _ ;
                                     the_struct }
                                ) + (({
                                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_empty_8_layoutinfo_v }
                                ) . apply_pin (_self) . get ())) ;
                                 ;
                                 {
                                     let mut the_struct = r#LayoutInfo :: default () ;
                                     the_struct . r#max = 80f64 as _ ;
                                     the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                     the_struct . r#min = 80f64 as _ ;
                                     the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                     the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                     the_struct . r#stretch = (r#layout_info . clone ()) . r#stretch as _ ;
                                     the_struct }
                                 }
                             as _ ;
                             the_struct }
                        ) ;
                         items_vec . push ({
                             let mut the_struct = r#BoxLayoutCellData :: default () ;
                             the_struct . r#constraint = {
                                 let r#layout_info = (({
                                     let mut the_struct = r#LayoutInfo :: default () ;
                                     the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                                     the_struct . r#max_percent = 100f64 as _ ;
                                     the_struct . r#min = 0f64 as _ ;
                                     the_struct . r#min_percent = 0f64 as _ ;
                                     the_struct . r#preferred = 0f64 as _ ;
                                     the_struct . r#stretch = 1f64 as _ ;
                                     the_struct }
                                ) + ((({
                                     let mut the_struct = r#LayoutInfo :: default () ;
                                     the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                                     the_struct . r#max_percent = 100f64 as _ ;
                                     the_struct . r#min = 0f64 as _ ;
                                     the_struct . r#min_percent = 0f64 as _ ;
                                     the_struct . r#preferred = 0f64 as _ ;
                                     the_struct . r#stretch = 1f64 as _ ;
                                     the_struct }
                                ) + (({
                                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_i_layout_13_layoutinfo_v }
                                ) . apply_pin (_self) . get ())))) ;
                                 ;
                                 {
                                     let mut the_struct = r#LayoutInfo :: default () ;
                                     the_struct . r#max = 45f64 as _ ;
                                     the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                     the_struct . r#min = 45f64 as _ ;
                                     the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                     the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                     the_struct . r#stretch = 0f64 as _ ;
                                     the_struct }
                                 }
                             as _ ;
                             the_struct }
                        ) ;
                         InnerMainWindow :: FIELD_OFFSETS . repeater0 . apply_pin (_self) . ensure_updated (|| {
                             InnerComponent_text_20 :: new (_self . self_weak . get () . unwrap () . clone ()) . into () }
                        ) ;
                         let internal_vec = _self . repeater0 . components_vec () ;
                         for sub_comp in & internal_vec {
                             items_vec . push (sub_comp . as_pin_ref () . box_layout_data (sp :: Orientation :: Vertical)) }
                         items_vec . push ({
                             let mut the_struct = r#BoxLayoutCellData :: default () ;
                             the_struct . r#constraint = {
                                 let r#layout_info = (({
                                     let mut the_struct = r#LayoutInfo :: default () ;
                                     the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                                     the_struct . r#max_percent = 100f64 as _ ;
                                     the_struct . r#min = 0f64 as _ ;
                                     the_struct . r#min_percent = 0f64 as _ ;
                                     the_struct . r#preferred = 0f64 as _ ;
                                     the_struct . r#stretch = 1f64 as _ ;
                                     the_struct }
                                ) + ((({
                                     let mut the_struct = r#LayoutInfo :: default () ;
                                     the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                                     the_struct . r#max_percent = 100f64 as _ ;
                                     the_struct . r#min = 0f64 as _ ;
                                     the_struct . r#min_percent = 0f64 as _ ;
                                     the_struct . r#preferred = 0f64 as _ ;
                                     the_struct . r#stretch = 1f64 as _ ;
                                     the_struct }
                                ) + (({
                                     InnerMainWindow :: FIELD_OFFSETS . r#button_22 }
                                 + {
                                     * & InnerButton_root_25 :: FIELD_OFFSETS . r#root_25_i_layout_28_layoutinfo_v }
                                ) . apply_pin (_self) . get ())))) ;
                                 ;
                                 {
                                     let mut the_struct = r#LayoutInfo :: default () ;
                                     the_struct . r#max = 45f64 as _ ;
                                     the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                     the_struct . r#min = 45f64 as _ ;
                                     the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                     the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                     the_struct . r#stretch = 0f64 as _ ;
                                     the_struct }
                                 }
                             as _ ;
                             the_struct }
                        ) ;
                         let r#cells = sp :: Slice :: from_slice (& items_vec) ;
                         r#box_layout_info (r#cells . clone () as _ , 24f64 as _ , & {
                             let mut the_struct = r#Padding :: default () ;
                             the_struct . r#begin = 40f64 as _ ;
                             the_struct . r#end = 40f64 as _ ;
                             the_struct }
                         as _ , sp :: r#LayoutAlignment :: r#Start as _) }
                    ) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_empty_3_layout_cache }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (r#solve_box_layout (& r#BoxLayoutData {
                         r#alignment : sp :: r#LayoutAlignment :: r#Stretch as _ , r#cells : Slice :: from_slice (& [{
                             let mut the_struct = r#BoxLayoutCellData :: default () ;
                             the_struct . r#constraint = sp :: Item :: layout_info (({
                                 * & InnerMainWindow :: FIELD_OFFSETS . r#text_4 }
                            ) . apply_pin (_self) , sp :: Orientation :: Vertical , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ())) as _ ;
                             the_struct }
                         , {
                             let mut the_struct = r#BoxLayoutCellData :: default () ;
                             the_struct . r#constraint = sp :: Item :: layout_info (({
                                 * & InnerMainWindow :: FIELD_OFFSETS . r#text_5 }
                            ) . apply_pin (_self) , sp :: Orientation :: Vertical , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ())) as _ ;
                             the_struct }
                        ]) as _ , r#padding : {
                             let mut the_struct = r#Padding :: default () ;
                             the_struct . r#begin = 8f64 as _ ;
                             the_struct . r#end = 8f64 as _ ;
                             the_struct }
                         as _ , r#size : ({
                             * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_empty_2_layout_cache }
                        ) . apply_pin (_self) . get () [1usize] as _ , r#spacing : 8f64 as _ , }
                     as _ , Slice :: from_slice (& []) as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_empty_3_layoutinfo_h }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (r#box_layout_info_ortho (Slice :: from_slice (& [{
                         let mut the_struct = r#BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = sp :: Item :: layout_info (({
                             * & InnerMainWindow :: FIELD_OFFSETS . r#text_4 }
                        ) . apply_pin (_self) , sp :: Orientation :: Horizontal , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ())) as _ ;
                         the_struct }
                     , {
                         let mut the_struct = r#BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = sp :: Item :: layout_info (({
                             * & InnerMainWindow :: FIELD_OFFSETS . r#text_5 }
                        ) . apply_pin (_self) , sp :: Orientation :: Horizontal , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ())) as _ ;
                         the_struct }
                    ]) as _ , & {
                         let mut the_struct = r#Padding :: default () ;
                         the_struct . r#begin = 8f64 as _ ;
                         the_struct . r#end = 8f64 as _ ;
                         the_struct }
                     as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_empty_3_layoutinfo_v }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (r#box_layout_info (Slice :: from_slice (& [{
                         let mut the_struct = r#BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = sp :: Item :: layout_info (({
                             * & InnerMainWindow :: FIELD_OFFSETS . r#text_4 }
                        ) . apply_pin (_self) , sp :: Orientation :: Vertical , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ())) as _ ;
                         the_struct }
                     , {
                         let mut the_struct = r#BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = sp :: Item :: layout_info (({
                             * & InnerMainWindow :: FIELD_OFFSETS . r#text_5 }
                        ) . apply_pin (_self) , sp :: Orientation :: Vertical , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ())) as _ ;
                         the_struct }
                    ]) as _ , 8f64 as _ , & {
                         let mut the_struct = r#Padding :: default () ;
                         the_struct . r#begin = 8f64 as _ ;
                         the_struct . r#end = 8f64 as _ ;
                         the_struct }
                     as _ , sp :: r#LayoutAlignment :: r#Stretch as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_empty_8_layout_cache }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (r#solve_box_layout (& r#BoxLayoutData {
                         r#alignment : sp :: r#LayoutAlignment :: r#Center as _ , r#cells : Slice :: from_slice (& [{
                             let mut the_struct = r#BoxLayoutCellData :: default () ;
                             the_struct . r#constraint = sp :: Item :: layout_info (({
                                 * & InnerMainWindow :: FIELD_OFFSETS . r#text_9 }
                            ) . apply_pin (_self) , sp :: Orientation :: Vertical , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ())) as _ ;
                             the_struct }
                         , {
                             let mut the_struct = r#BoxLayoutCellData :: default () ;
                             the_struct . r#constraint = sp :: Item :: layout_info (({
                                 * & InnerMainWindow :: FIELD_OFFSETS . r#text_10 }
                            ) . apply_pin (_self) , sp :: Orientation :: Vertical , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ())) as _ ;
                             the_struct }
                        ]) as _ , r#padding : {
                             let mut the_struct = r#Padding :: default () ;
                             the_struct . r#begin = 8f64 as _ ;
                             the_struct . r#end = 8f64 as _ ;
                             the_struct }
                         as _ , r#size : 80f64 as _ , r#spacing : 8f64 as _ , }
                     as _ , Slice :: from_slice (& []) as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_empty_8_layoutinfo_h }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (r#box_layout_info_ortho (Slice :: from_slice (& [{
                         let mut the_struct = r#BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = sp :: Item :: layout_info (({
                             * & InnerMainWindow :: FIELD_OFFSETS . r#text_9 }
                        ) . apply_pin (_self) , sp :: Orientation :: Horizontal , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ())) as _ ;
                         the_struct }
                     , {
                         let mut the_struct = r#BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = sp :: Item :: layout_info (({
                             * & InnerMainWindow :: FIELD_OFFSETS . r#text_10 }
                        ) . apply_pin (_self) , sp :: Orientation :: Horizontal , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ())) as _ ;
                         the_struct }
                    ]) as _ , & {
                         let mut the_struct = r#Padding :: default () ;
                         the_struct . r#begin = 8f64 as _ ;
                         the_struct . r#end = 8f64 as _ ;
                         the_struct }
                     as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_empty_8_layoutinfo_v }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (r#box_layout_info (Slice :: from_slice (& [{
                         let mut the_struct = r#BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = sp :: Item :: layout_info (({
                             * & InnerMainWindow :: FIELD_OFFSETS . r#text_9 }
                        ) . apply_pin (_self) , sp :: Orientation :: Vertical , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ())) as _ ;
                         the_struct }
                     , {
                         let mut the_struct = r#BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = sp :: Item :: layout_info (({
                             * & InnerMainWindow :: FIELD_OFFSETS . r#text_10 }
                        ) . apply_pin (_self) , sp :: Orientation :: Vertical , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ())) as _ ;
                         the_struct }
                    ]) as _ , 8f64 as _ , & {
                         let mut the_struct = r#Padding :: default () ;
                         the_struct . r#begin = 8f64 as _ ;
                         the_struct . r#end = 8f64 as _ ;
                         the_struct }
                     as _ , sp :: r#LayoutAlignment :: r#Center as _)) as _ }
                ) ;
                 }
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_error_message }
            ) . apply_pin (_self) . set ({
                 (sp :: SharedString :: from ("")) as sp :: SharedString }
            ) ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#root_1 }
             + r#WindowItem :: FIELD_OFFSETS . r#height) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (480f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_i_layout_13_layout_cache }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (r#solve_box_layout (& r#BoxLayoutData {
                         r#alignment : sp :: r#LayoutAlignment :: r#Stretch as _ , r#cells : Slice :: from_slice (& [{
                             let mut the_struct = r#BoxLayoutCellData :: default () ;
                             the_struct . r#constraint = ({
                                 * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_rectangle_14_layoutinfo_h }
                            ) . apply_pin (_self) . get () as _ ;
                             the_struct }
                        ]) as _ , r#padding : {
                             let mut the_struct = r#Padding :: default () ;
                             the_struct . r#begin = 12f64 as _ ;
                             the_struct . r#end = 12f64 as _ ;
                             the_struct }
                         as _ , r#size : ({
                             * & InnerMainWindow :: FIELD_OFFSETS . r#i_background_12 }
                         + r#BorderRectangle :: FIELD_OFFSETS . r#width) . apply_pin (_self) . get () . get () as _ , r#spacing : 0f64 as _ , }
                     as _ , Slice :: from_slice (& []) as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_i_layout_13_layoutinfo_h }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (r#box_layout_info (Slice :: from_slice (& [{
                         let mut the_struct = r#BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = ({
                             * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_rectangle_14_layoutinfo_h }
                        ) . apply_pin (_self) . get () as _ ;
                         the_struct }
                    ]) as _ , 0f64 as _ , & {
                         let mut the_struct = r#Padding :: default () ;
                         the_struct . r#begin = 12f64 as _ ;
                         the_struct . r#end = 12f64 as _ ;
                         the_struct }
                     as _ , sp :: r#LayoutAlignment :: r#Stretch as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_i_layout_13_layoutinfo_v }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (r#box_layout_info_ortho (Slice :: from_slice (& [{
                         let mut the_struct = r#BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = ({
                             * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_rectangle_14_layoutinfo_v }
                        ) . apply_pin (_self) . get () as _ ;
                         the_struct }
                    ]) as _ , & {
                         let mut the_struct = r#Padding :: default () ;
                         the_struct . r#begin = 0f64 as _ ;
                         the_struct . r#end = 0f64 as _ ;
                         the_struct }
                     as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_i_placeholder_17_horizontal_stretch }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ((sp :: Item :: layout_info (({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#i_placeholder_17 }
                    ) . apply_pin (_self) , sp :: Orientation :: Horizontal , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()))) . r#stretch) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_i_placeholder_17_max_height }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((sp :: Item :: layout_info (({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#i_placeholder_17 }
                    ) . apply_pin (_self) , sp :: Orientation :: Vertical , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()))) . r#max as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_i_placeholder_17_max_width }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((sp :: Item :: layout_info (({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#i_placeholder_17 }
                    ) . apply_pin (_self) , sp :: Orientation :: Horizontal , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()))) . r#max as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_i_placeholder_17_min_height }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((sp :: Item :: layout_info (({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#i_placeholder_17 }
                    ) . apply_pin (_self) , sp :: Orientation :: Vertical , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()))) . r#min as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_i_placeholder_17_min_width }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((sp :: Item :: layout_info (({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#i_placeholder_17 }
                    ) . apply_pin (_self) , sp :: Orientation :: Horizontal , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()))) . r#min as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_i_placeholder_17_preferred_height }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((sp :: Item :: layout_info (({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#i_placeholder_17 }
                    ) . apply_pin (_self) , sp :: Orientation :: Vertical , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()))) . r#preferred as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_i_placeholder_17_preferred_width }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((sp :: Item :: layout_info (({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#i_placeholder_17 }
                    ) . apply_pin (_self) , sp :: Orientation :: Horizontal , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()))) . r#preferred as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_i_placeholder_17_vertical_stretch }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ((sp :: Item :: layout_info (({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#i_placeholder_17 }
                    ) . apply_pin (_self) , sp :: Orientation :: Vertical , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()))) . r#stretch) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_i_placeholder_17_visible }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (((({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#i_text_input_18 }
                     + r#TextInput :: FIELD_OFFSETS . r#text) . apply_pin (_self) . get ()) == (sp :: SharedString :: from ("")))) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_i_text_input_18_preferred_width }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((sp :: Item :: layout_info (({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#i_text_input_18 }
                    ) . apply_pin (_self) , sp :: Orientation :: Horizontal , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()))) . r#preferred as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_is_connected }
            ) . apply_pin (_self) . set ({
                 (false) as bool }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_layoutinfo_h }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (((sp :: Item :: layout_info (({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#root_1 }
                    ) . apply_pin (_self) , sp :: Orientation :: Horizontal , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()))) + (({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_empty_2_layoutinfo_h }
                    ) . apply_pin (_self) . get ()))) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_layoutinfo_v }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (((sp :: Item :: layout_info (({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#root_1 }
                    ) . apply_pin (_self) , sp :: Orientation :: Vertical , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()))) + (({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_empty_2_layoutinfo_v }
                    ) . apply_pin (_self) . get ()))) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_rectangle_14_layoutinfo_h }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ((({
                         let mut the_struct = r#LayoutInfo :: default () ;
                         the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                         the_struct . r#max_percent = 100f64 as _ ;
                         the_struct . r#min = 0f64 as _ ;
                         the_struct . r#min_percent = 0f64 as _ ;
                         the_struct . r#preferred = 0f64 as _ ;
                         the_struct . r#stretch = 1f64 as _ ;
                         the_struct }
                    ) + ({
                         let mut the_struct = r#LayoutInfo :: default () ;
                         the_struct . r#max = ({
                             * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_i_placeholder_17_max_width }
                        ) . apply_pin (_self) . get () . get () as _ ;
                         the_struct . r#max_percent = 100f64 as _ ;
                         the_struct . r#min = ({
                             * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_i_placeholder_17_min_width }
                        ) . apply_pin (_self) . get () . get () as _ ;
                         the_struct . r#min_percent = 0f64 as _ ;
                         the_struct . r#preferred = ({
                             * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_i_placeholder_17_preferred_width }
                        ) . apply_pin (_self) . get () . get () as _ ;
                         the_struct . r#stretch = ({
                             * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_i_placeholder_17_horizontal_stretch }
                        ) . apply_pin (_self) . get () as _ ;
                         the_struct }
                    ))) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_rectangle_14_layoutinfo_v }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ((({
                         let mut the_struct = r#LayoutInfo :: default () ;
                         the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                         the_struct . r#max_percent = 100f64 as _ ;
                         the_struct . r#min = 0f64 as _ ;
                         the_struct . r#min_percent = 0f64 as _ ;
                         the_struct . r#preferred = 0f64 as _ ;
                         the_struct . r#stretch = 1f64 as _ ;
                         the_struct }
                    ) + ({
                         let mut the_struct = r#LayoutInfo :: default () ;
                         the_struct . r#max = ({
                             * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_i_placeholder_17_max_height }
                        ) . apply_pin (_self) . get () . get () as _ ;
                         the_struct . r#max_percent = 100f64 as _ ;
                         the_struct . r#min = ({
                             * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_i_placeholder_17_min_height }
                        ) . apply_pin (_self) . get () . get () as _ ;
                         the_struct . r#min_percent = 0f64 as _ ;
                         the_struct . r#preferred = ({
                             * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_i_placeholder_17_preferred_height }
                        ) . apply_pin (_self) . get () . get () as _ ;
                         the_struct . r#stretch = ({
                             * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_i_placeholder_17_vertical_stretch }
                        ) . apply_pin (_self) . get () as _ ;
                         the_struct }
                    ))) as _ }
                ) ;
                 }
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#root_1 }
             + r#WindowItem :: FIELD_OFFSETS . r#title) . apply_pin (_self) . set ({
                 (sp :: SharedString :: from ("Subnetra 客户端")) as sp :: SharedString }
            ) ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#root_1 }
             + r#WindowItem :: FIELD_OFFSETS . r#width) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (380f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#empty_3 }
                 + r#Empty :: FIELD_OFFSETS . r#height) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_empty_2_layout_cache }
                    ) . apply_pin (_self) . get () [1usize] as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#empty_3 }
                 + r#Empty :: FIELD_OFFSETS . r#width) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ({
                         let r#tmp_empty_2_padding = 40f64 ;
                         ;
                         ((((({
                             * & InnerMainWindow :: FIELD_OFFSETS . r#root_1 }
                         + r#WindowItem :: FIELD_OFFSETS . r#width) . apply_pin (_self) . get () . get () as f64) - (r#tmp_empty_2_padding . clone () as f64)) as f64) - (r#tmp_empty_2_padding . clone () as f64)) }
                     as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#empty_3 }
             + r#Empty :: FIELD_OFFSETS . r#x) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (40f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#empty_3 }
                 + r#Empty :: FIELD_OFFSETS . r#y) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_empty_2_layout_cache }
                    ) . apply_pin (_self) . get () [0usize] as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#text_4 }
             + r#Text :: FIELD_OFFSETS . r#color) . apply_pin (_self) . set ({
                 (slint :: Brush :: SolidColor (sp :: Color :: from_argb_encoded (4281908728f64 as u32))) as slint :: Brush }
            ) ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#text_4 }
             + r#Text :: FIELD_OFFSETS . r#font_size) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (32f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#text_4 }
             + r#Text :: FIELD_OFFSETS . r#font_weight) . apply_pin (_self) . set ({
                 (800f64) as i32 }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#text_4 }
                 + r#Text :: FIELD_OFFSETS . r#height) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_empty_3_layout_cache }
                    ) . apply_pin (_self) . get () [1usize] as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#text_4 }
             + r#Text :: FIELD_OFFSETS . r#horizontal_alignment) . apply_pin (_self) . set ({
                 (sp :: r#TextHorizontalAlignment :: r#Center) as sp :: r#TextHorizontalAlignment }
            ) ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#text_4 }
             + r#Text :: FIELD_OFFSETS . r#text) . apply_pin (_self) . set ({
                 (sp :: SharedString :: from ("Subnetra")) as sp :: SharedString }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#text_4 }
                 + r#Text :: FIELD_OFFSETS . r#width) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ({
                         let r#tmp_empty_3_padding = 8f64 ;
                         ;
                         (((({
                             let r#tmp_empty_2_padding = 40f64 ;
                             ;
                             ((((({
                                 * & InnerMainWindow :: FIELD_OFFSETS . r#root_1 }
                             + r#WindowItem :: FIELD_OFFSETS . r#width) . apply_pin (_self) . get () . get () as f64) - (r#tmp_empty_2_padding . clone () as f64)) as f64) - (r#tmp_empty_2_padding . clone () as f64)) }
                         as f64) - (r#tmp_empty_3_padding . clone () as f64)) as f64) - (r#tmp_empty_3_padding . clone () as f64)) }
                     as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#text_4 }
             + r#Text :: FIELD_OFFSETS . r#x) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (8f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#text_4 }
                 + r#Text :: FIELD_OFFSETS . r#y) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_empty_3_layout_cache }
                    ) . apply_pin (_self) . get () [0usize] as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#text_5 }
             + r#Text :: FIELD_OFFSETS . r#color) . apply_pin (_self) . set ({
                 (slint :: Brush :: SolidColor (sp :: Color :: from_argb_encoded (4284773515f64 as u32))) as slint :: Brush }
            ) ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#text_5 }
             + r#Text :: FIELD_OFFSETS . r#font_size) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (14f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#text_5 }
                 + r#Text :: FIELD_OFFSETS . r#height) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_empty_3_layout_cache }
                    ) . apply_pin (_self) . get () [3usize] as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#text_5 }
             + r#Text :: FIELD_OFFSETS . r#horizontal_alignment) . apply_pin (_self) . set ({
                 (sp :: r#TextHorizontalAlignment :: r#Center) as sp :: r#TextHorizontalAlignment }
            ) ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#text_5 }
             + r#Text :: FIELD_OFFSETS . r#text) . apply_pin (_self) . set ({
                 (sp :: SharedString :: from ("企业级虚拟安全网络")) as sp :: SharedString }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#text_5 }
                 + r#Text :: FIELD_OFFSETS . r#width) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ({
                         let r#tmp_empty_3_padding = 8f64 ;
                         ;
                         (((({
                             let r#tmp_empty_2_padding = 40f64 ;
                             ;
                             ((((({
                                 * & InnerMainWindow :: FIELD_OFFSETS . r#root_1 }
                             + r#WindowItem :: FIELD_OFFSETS . r#width) . apply_pin (_self) . get () . get () as f64) - (r#tmp_empty_2_padding . clone () as f64)) as f64) - (r#tmp_empty_2_padding . clone () as f64)) }
                         as f64) - (r#tmp_empty_3_padding . clone () as f64)) as f64) - (r#tmp_empty_3_padding . clone () as f64)) }
                     as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#text_5 }
             + r#Text :: FIELD_OFFSETS . r#x) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (8f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#text_5 }
                 + r#Text :: FIELD_OFFSETS . r#y) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_empty_3_layout_cache }
                    ) . apply_pin (_self) . get () [2usize] as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#_shadow_6 }
                 + r#BoxShadow :: FIELD_OFFSETS . r#blur) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (if ({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_is_connected }
                    ) . apply_pin (_self) . get () {
                         15f64 }
                     else {
                         (0f64) as _ }
                     as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#_shadow_6 }
             + r#BoxShadow :: FIELD_OFFSETS . r#border_radius) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (12f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#_shadow_6 }
                 + r#BoxShadow :: FIELD_OFFSETS . r#color) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (if ({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_is_connected }
                    ) . apply_pin (_self) . get () {
                         sp :: Color :: from_argb_encoded (1430969984f64 as u32) }
                     else {
                         (sp :: Color :: from_argb_encoded (0f64 as u32)) as _ }
                    ) as _ }
                ) ;
                 }
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#_shadow_6 }
             + r#BoxShadow :: FIELD_OFFSETS . r#height) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (80f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#_shadow_6 }
                 + r#BoxShadow :: FIELD_OFFSETS . r#width) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ({
                         let r#tmp_empty_2_padding = 40f64 ;
                         ;
                         ((((({
                             * & InnerMainWindow :: FIELD_OFFSETS . r#root_1 }
                         + r#WindowItem :: FIELD_OFFSETS . r#width) . apply_pin (_self) . get () . get () as f64) - (r#tmp_empty_2_padding . clone () as f64)) as f64) - (r#tmp_empty_2_padding . clone () as f64)) }
                     as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#_shadow_6 }
             + r#BoxShadow :: FIELD_OFFSETS . r#x) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (40f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#_shadow_6 }
                 + r#BoxShadow :: FIELD_OFFSETS . r#y) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_empty_2_layout_cache }
                    ) . apply_pin (_self) . get () [2usize] as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#rectangle_7 }
             + r#BorderRectangle :: FIELD_OFFSETS . r#background) . apply_pin (_self) . set ({
                 (slint :: Brush :: SolidColor (sp :: Color :: from_argb_encoded (4280166715f64 as u32))) as slint :: Brush }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#rectangle_7 }
                 + r#BorderRectangle :: FIELD_OFFSETS . r#border_color) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: Brush :: SolidColor (if ({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_is_connected }
                    ) . apply_pin (_self) . get () {
                         sp :: Color :: from_argb_encoded (4283096704f64 as u32) }
                     else {
                         (sp :: Color :: from_argb_encoded (4282865001f64 as u32)) as _ }
                    )) as _ }
                ) ;
                 }
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#rectangle_7 }
             + r#BorderRectangle :: FIELD_OFFSETS . r#border_radius) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (12f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#rectangle_7 }
             + r#BorderRectangle :: FIELD_OFFSETS . r#border_width) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (1f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#rectangle_7 }
             + r#BorderRectangle :: FIELD_OFFSETS . r#height) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (80f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#rectangle_7 }
                 + r#BorderRectangle :: FIELD_OFFSETS . r#width) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ({
                         let r#tmp_empty_2_padding = 40f64 ;
                         ;
                         ((((({
                             * & InnerMainWindow :: FIELD_OFFSETS . r#root_1 }
                         + r#WindowItem :: FIELD_OFFSETS . r#width) . apply_pin (_self) . get () . get () as f64) - (r#tmp_empty_2_padding . clone () as f64)) as f64) - (r#tmp_empty_2_padding . clone () as f64)) }
                     as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#rectangle_7 }
             + r#BorderRectangle :: FIELD_OFFSETS . r#x) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (40f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#rectangle_7 }
                 + r#BorderRectangle :: FIELD_OFFSETS . r#y) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_empty_2_layout_cache }
                    ) . apply_pin (_self) . get () [2usize] as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#text_9 }
                 + r#Text :: FIELD_OFFSETS . r#color) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: Brush :: SolidColor (if ({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_is_connected }
                    ) . apply_pin (_self) . get () {
                         sp :: Color :: from_argb_encoded (4283096704f64 as u32) }
                     else {
                         (sp :: Color :: from_argb_encoded (4287931320f64 as u32)) as _ }
                    )) as _ }
                ) ;
                 }
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#text_9 }
             + r#Text :: FIELD_OFFSETS . r#font_size) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (20f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#text_9 }
             + r#Text :: FIELD_OFFSETS . r#font_weight) . apply_pin (_self) . set ({
                 (700f64) as i32 }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#text_9 }
                 + r#Text :: FIELD_OFFSETS . r#height) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_empty_8_layout_cache }
                    ) . apply_pin (_self) . get () [1usize] as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#text_9 }
             + r#Text :: FIELD_OFFSETS . r#horizontal_alignment) . apply_pin (_self) . set ({
                 (sp :: r#TextHorizontalAlignment :: r#Center) as sp :: r#TextHorizontalAlignment }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#text_9 }
                 + r#Text :: FIELD_OFFSETS . r#text) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (if ({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_is_connected }
                    ) . apply_pin (_self) . get () {
                         sp :: SharedString :: from ("已连接") }
                     else {
                         (sp :: SharedString :: from ("未连接")) as _ }
                    ) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#text_9 }
                 + r#Text :: FIELD_OFFSETS . r#width) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ({
                         let r#tmp_empty_8_padding = 8f64 ;
                         ;
                         (((({
                             let r#tmp_empty_2_padding = 40f64 ;
                             ;
                             ((((({
                                 * & InnerMainWindow :: FIELD_OFFSETS . r#root_1 }
                             + r#WindowItem :: FIELD_OFFSETS . r#width) . apply_pin (_self) . get () . get () as f64) - (r#tmp_empty_2_padding . clone () as f64)) as f64) - (r#tmp_empty_2_padding . clone () as f64)) }
                         as f64) - (r#tmp_empty_8_padding . clone () as f64)) as f64) - (r#tmp_empty_8_padding . clone () as f64)) }
                     as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#text_9 }
             + r#Text :: FIELD_OFFSETS . r#x) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (8f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#text_9 }
                 + r#Text :: FIELD_OFFSETS . r#y) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_empty_8_layout_cache }
                    ) . apply_pin (_self) . get () [0usize] as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#text_10 }
             + r#Text :: FIELD_OFFSETS . r#color) . apply_pin (_self) . set ({
                 (slint :: Brush :: SolidColor (sp :: Color :: from_argb_encoded (4287931320f64 as u32))) as slint :: Brush }
            ) ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#text_10 }
             + r#Text :: FIELD_OFFSETS . r#font_size) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (12f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#text_10 }
                 + r#Text :: FIELD_OFFSETS . r#height) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_empty_8_layout_cache }
                    ) . apply_pin (_self) . get () [3usize] as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#text_10 }
             + r#Text :: FIELD_OFFSETS . r#horizontal_alignment) . apply_pin (_self) . set ({
                 (sp :: r#TextHorizontalAlignment :: r#Center) as sp :: r#TextHorizontalAlignment }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#text_10 }
                 + r#Text :: FIELD_OFFSETS . r#text) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (((sp :: SharedString :: from ("当前状态: ")) + (({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_connection_status }
                    ) . apply_pin (_self) . get () . as_str ()))) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#text_10 }
                 + r#Text :: FIELD_OFFSETS . r#width) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ({
                         let r#tmp_empty_8_padding = 8f64 ;
                         ;
                         (((({
                             let r#tmp_empty_2_padding = 40f64 ;
                             ;
                             ((((({
                                 * & InnerMainWindow :: FIELD_OFFSETS . r#root_1 }
                             + r#WindowItem :: FIELD_OFFSETS . r#width) . apply_pin (_self) . get () . get () as f64) - (r#tmp_empty_2_padding . clone () as f64)) as f64) - (r#tmp_empty_2_padding . clone () as f64)) }
                         as f64) - (r#tmp_empty_8_padding . clone () as f64)) as f64) - (r#tmp_empty_8_padding . clone () as f64)) }
                     as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#text_10 }
             + r#Text :: FIELD_OFFSETS . r#x) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (8f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#text_10 }
                 + r#Text :: FIELD_OFFSETS . r#y) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_empty_8_layout_cache }
                    ) . apply_pin (_self) . get () [2usize] as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#auth_input_11 }
             + r#Empty :: FIELD_OFFSETS . r#height) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (45f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#auth_input_11 }
                 + r#Empty :: FIELD_OFFSETS . r#width) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ({
                         let r#tmp_empty_2_padding = 40f64 ;
                         ;
                         ((((({
                             * & InnerMainWindow :: FIELD_OFFSETS . r#root_1 }
                         + r#WindowItem :: FIELD_OFFSETS . r#width) . apply_pin (_self) . get () . get () as f64) - (r#tmp_empty_2_padding . clone () as f64)) as f64) - (r#tmp_empty_2_padding . clone () as f64)) }
                     as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#auth_input_11 }
             + r#Empty :: FIELD_OFFSETS . r#x) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (40f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#auth_input_11 }
                 + r#Empty :: FIELD_OFFSETS . r#y) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_empty_2_layout_cache }
                    ) . apply_pin (_self) . get () [4usize] as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#i_background_12 }
                 + r#BorderRectangle :: FIELD_OFFSETS . r#background) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ({
                         let r#tmp_auth_input_11_state = ({
                             * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_auth_input_11_state }
                        ) . apply_pin (_self) . get () ;
                         ;
                         if ((r#tmp_auth_input_11_state . clone () as f64) == (1f64 as f64)) {
                             slint :: Brush :: SolidColor (if InnerColorSchemeSelector_37 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . root . get () . unwrap () . upgrade () . unwrap () . globals . global_ColorSchemeSelector_37 . as_ref ()) . get () {
                                 sp :: Color :: from_argb_encoded (184549375f64 as u32) }
                             else {
                                 (sp :: Color :: from_argb_encoded (1308228089f64 as u32)) as _ }
                            ) }
                         else {
                             (if ((r#tmp_auth_input_11_state . clone () as f64) == (2f64 as f64)) {
                                 slint :: Brush :: SolidColor (if InnerColorSchemeSelector_37 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . root . get () . unwrap () . upgrade () . unwrap () . globals . global_ColorSchemeSelector_37 . as_ref ()) . get () {
                                     sp :: Color :: from_argb_encoded (3005095454f64 as u32) }
                                 else {
                                     (sp :: Color :: from_argb_encoded (4294967295f64 as u32)) as _ }
                                ) }
                             else {
                                 (slint :: Brush :: SolidColor (if InnerColorSchemeSelector_37 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . root . get () . unwrap () . upgrade () . unwrap () . globals . global_ColorSchemeSelector_37 . as_ref ()) . get () {
                                     sp :: Color :: from_argb_encoded (268435455f64 as u32) }
                                 else {
                                     (sp :: Color :: from_argb_encoded (3019898879f64 as u32)) as _ }
                                )) as _ }
                            ) as _ }
                         }
                    ) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#i_background_12 }
                 + r#BorderRectangle :: FIELD_OFFSETS . r#border_color) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ({
                         let r#tmp_auth_input_11_state = ({
                             * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_auth_input_11_state }
                        ) . apply_pin (_self) . get () ;
                         ;
                         if ((r#tmp_auth_input_11_state . clone () as f64) == (1f64 as f64)) {
                             slint :: Brush :: SolidColor (if InnerColorSchemeSelector_37 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . root . get () . unwrap () . upgrade () . unwrap () . globals . global_ColorSchemeSelector_37 . as_ref ()) . get () {
                                 sp :: Color :: from_argb_encoded (352321535f64 as u32) }
                             else {
                                 (sp :: Color :: from_argb_encoded (251658240f64 as u32)) as _ }
                            ) }
                         else {
                             (if ((r#tmp_auth_input_11_state . clone () as f64) == (2f64 as f64)) {
                                 slint :: Brush :: SolidColor (if InnerColorSchemeSelector_37 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . root . get () . unwrap () . upgrade () . unwrap () . globals . global_ColorSchemeSelector_37 . as_ref ()) . get () {
                                     sp :: Color :: from_argb_encoded (352321535f64 as u32) }
                                 else {
                                     (sp :: Color :: from_argb_encoded (251658240f64 as u32)) as _ }
                                ) }
                             else {
                                 (InnerPalette_38 :: FIELD_OFFSETS . r#text_control_border . apply_pin (_self . root . get () . unwrap () . upgrade () . unwrap () . globals . global_Palette_38 . as_ref ()) . get ()) as _ }
                            ) as _ }
                         }
                    ) as _ }
                ) ;
                 }
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#i_background_12 }
             + r#BorderRectangle :: FIELD_OFFSETS . r#border_radius) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (4f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#i_background_12 }
             + r#BorderRectangle :: FIELD_OFFSETS . r#border_width) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (1f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#i_background_12 }
             + r#BorderRectangle :: FIELD_OFFSETS . r#height) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (45f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#i_background_12 }
                 + r#BorderRectangle :: FIELD_OFFSETS . r#width) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ({
                         let r#tmp_empty_2_padding = 40f64 ;
                         ;
                         ((((({
                             * & InnerMainWindow :: FIELD_OFFSETS . r#root_1 }
                         + r#WindowItem :: FIELD_OFFSETS . r#width) . apply_pin (_self) . get () . get () as f64) - (r#tmp_empty_2_padding . clone () as f64)) as f64) - (r#tmp_empty_2_padding . clone () as f64)) }
                     as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#rectangle_14 }
             + r#Empty :: FIELD_OFFSETS . r#height) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (45f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#rectangle_14 }
                 + r#Empty :: FIELD_OFFSETS . r#width) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_i_layout_13_layout_cache }
                    ) . apply_pin (_self) . get () [1usize] as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#rectangle_14 }
                 + r#Empty :: FIELD_OFFSETS . r#x) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_i_layout_13_layout_cache }
                    ) . apply_pin (_self) . get () [0usize] as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#_clip_15 }
                 + r#Clip :: FIELD_OFFSETS . r#clip) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (true) as _ }
                ) ;
                 }
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#_clip_15 }
             + r#Clip :: FIELD_OFFSETS . r#height) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (45f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#_clip_15 }
                 + r#Clip :: FIELD_OFFSETS . r#width) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_i_layout_13_layout_cache }
                    ) . apply_pin (_self) . get () [1usize] as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#i_placeholder_visibility_16 }
                 + r#Clip :: FIELD_OFFSETS . r#clip) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (! ({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_i_placeholder_17_visible }
                    ) . apply_pin (_self) . get ()) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#i_placeholder_17 }
                 + r#Text :: FIELD_OFFSETS . r#color) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ({
                         let r#tmp_auth_input_11_state = ({
                             * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_auth_input_11_state }
                        ) . apply_pin (_self) . get () ;
                         ;
                         if ((r#tmp_auth_input_11_state . clone () as f64) == (1f64 as f64)) {
                             slint :: Brush :: SolidColor (if InnerColorSchemeSelector_37 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . root . get () . unwrap () . upgrade () . unwrap () . globals . global_ColorSchemeSelector_37 . as_ref ()) . get () {
                                 sp :: Color :: from_argb_encoded (1593835519f64 as u32) }
                             else {
                                 (sp :: Color :: from_argb_encoded (1577058304f64 as u32)) as _ }
                            ) }
                         else {
                             (if ((r#tmp_auth_input_11_state . clone () as f64) == (2f64 as f64)) {
                                 slint :: Brush :: SolidColor (if InnerColorSchemeSelector_37 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . root . get () . unwrap () . upgrade () . unwrap () . globals . global_ColorSchemeSelector_37 . as_ref ()) . get () {
                                     sp :: Color :: from_argb_encoded (2332033023f64 as u32) }
                                 else {
                                     (sp :: Color :: from_argb_encoded (1929379840f64 as u32)) as _ }
                                ) }
                             else {
                                 (slint :: Brush :: SolidColor (if InnerColorSchemeSelector_37 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . root . get () . unwrap () . upgrade () . unwrap () . globals . global_ColorSchemeSelector_37 . as_ref ()) . get () {
                                     sp :: Color :: from_argb_encoded (3388997631f64 as u32) }
                                 else {
                                     (sp :: Color :: from_argb_encoded (2566914048f64 as u32)) as _ }
                                )) as _ }
                            ) as _ }
                         }
                    ) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#i_placeholder_17 }
                 + r#Text :: FIELD_OFFSETS . r#font_size) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (((({
                         let obj = (1.0766f64 as f32 , 400f64 as i32 ,) ;
                         let mut the_struct = r#TextStyle :: default () ;
                         the_struct . r#font_size = obj . 0 as _ ;
                         the_struct . r#font_weight = obj . 1 as _ ;
                         the_struct }
                    ) . r#font_size as f64) * (({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#root_1 }
                     + sp :: WindowItem :: FIELD_OFFSETS . default_font_size) . apply_pin (_self . root . get () . unwrap () . upgrade () . unwrap () . as_pin_ref ()) . get () . get () as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#i_placeholder_17 }
             + r#Text :: FIELD_OFFSETS . r#font_weight) . apply_pin (_self) . set ({
                 (({
                     let obj = (1.0766f64 as f32 , 400f64 as i32 ,) ;
                     let mut the_struct = r#TextStyle :: default () ;
                     the_struct . r#font_size = obj . 0 as _ ;
                     the_struct . r#font_weight = obj . 1 as _ ;
                     the_struct }
                ) . r#font_weight) as i32 }
            ) ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#i_placeholder_17 }
             + r#Text :: FIELD_OFFSETS . r#height) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (((1f64 as f64) * (45f64 as f64)) as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#i_placeholder_17 }
             + r#Text :: FIELD_OFFSETS . r#text) . apply_pin (_self) . set ({
                 (sp :: SharedString :: from ("粘贴邀请码 (Auth Key)")) as sp :: SharedString }
            ) ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#i_placeholder_17 }
             + r#Text :: FIELD_OFFSETS . r#vertical_alignment) . apply_pin (_self) . set ({
                 (sp :: r#TextVerticalAlignment :: r#Center) as sp :: r#TextVerticalAlignment }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#i_placeholder_17 }
                 + r#Text :: FIELD_OFFSETS . r#width) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (((1f64 as f64) * (({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_i_layout_13_layout_cache }
                    ) . apply_pin (_self) . get () [1usize] as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#i_text_input_18 }
                 + r#TextInput :: FIELD_OFFSETS . r#accepted) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             ({
                                 * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_auth_input_11_accepted }
                            ) . apply_pin (_self) . call (& (({
                                 * & InnerMainWindow :: FIELD_OFFSETS . r#i_text_input_18 }
                             + r#TextInput :: FIELD_OFFSETS . r#text) . apply_pin (_self) . get () as _ ,) . into ()) }
                        ) ;
                         }
                     }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#i_text_input_18 }
                 + r#TextInput :: FIELD_OFFSETS . r#color) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (if ((({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_auth_input_11_state }
                    ) . apply_pin (_self) . get () as f64) == (1f64 as f64)) {
                         slint :: Brush :: SolidColor (if InnerColorSchemeSelector_37 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . root . get () . unwrap () . upgrade () . unwrap () . globals . global_ColorSchemeSelector_37 . as_ref ()) . get () {
                             sp :: Color :: from_argb_encoded (1593835519f64 as u32) }
                         else {
                             (sp :: Color :: from_argb_encoded (1577058304f64 as u32)) as _ }
                        ) }
                     else {
                         (slint :: Brush :: SolidColor (if InnerColorSchemeSelector_37 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . root . get () . unwrap () . upgrade () . unwrap () . globals . global_ColorSchemeSelector_37 . as_ref ()) . get () {
                             sp :: Color :: from_argb_encoded (4294967295f64 as u32) }
                         else {
                             (sp :: Color :: from_argb_encoded (3858759680f64 as u32)) as _ }
                        )) as _ }
                    ) as _ }
                ) ;
                 }
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#i_text_input_18 }
                 + r#TextInput :: FIELD_OFFSETS . r#cursor_position_changed) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             if (((((args . 0 . clone ()) . r#x as f64) + (({
                                 * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_i_text_input_18_computed_x }
                            ) . apply_pin (_self) . get () . get () as f64)) as f64) < (24f64 as f64)) {
                                 {
                                     ({
                                         * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_i_text_input_18_computed_x }
                                    ) . apply_pin (_self) . set (sp :: LogicalLength :: new (((- (args . 0 . clone ()) . r#x as f64) + (24f64 as f64)) as sp :: Coord) as _) }
                                 }
                             else {
                                 (if (((((args . 0 . clone ()) . r#x as f64) + (({
                                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_i_text_input_18_computed_x }
                                ) . apply_pin (_self) . get () . get () as f64)) as f64) > (((({
                                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_i_layout_13_layout_cache }
                                ) . apply_pin (_self) . get () [1usize] as f64) - (24f64 as f64)) as f64)) {
                                     {
                                         ({
                                             * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_i_text_input_18_computed_x }
                                        ) . apply_pin (_self) . set (sp :: LogicalLength :: new (((((({
                                             * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_i_layout_13_layout_cache }
                                        ) . apply_pin (_self) . get () [1usize] as f64) - ((args . 0 . clone ()) . r#x as f64)) as f64) - (24f64 as f64)) as sp :: Coord) as _) }
                                     }
                                 else {
                                     ({
                                         }
                                    ) as _ }
                                ) as _ }
                             }
                        ) ;
                         }
                     }
                ) ;
                 }
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#i_text_input_18 }
                 + r#TextInput :: FIELD_OFFSETS . r#edited) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             ({
                                 * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_auth_input_11_edited }
                            ) . apply_pin (_self) . call (& (({
                                 * & InnerMainWindow :: FIELD_OFFSETS . r#i_text_input_18 }
                             + r#TextInput :: FIELD_OFFSETS . r#text) . apply_pin (_self) . get () as _ ,) . into ()) }
                        ) ;
                         }
                     }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#i_text_input_18 }
                 + r#TextInput :: FIELD_OFFSETS . r#enabled) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (! ({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_is_connected }
                    ) . apply_pin (_self) . get ()) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#i_text_input_18 }
                 + r#TextInput :: FIELD_OFFSETS . r#font_size) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (14f64 as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#i_text_input_18 }
             + r#TextInput :: FIELD_OFFSETS . r#font_weight) . apply_pin (_self) . set ({
                 (({
                     let obj = (1.0766f64 as f32 , 400f64 as i32 ,) ;
                     let mut the_struct = r#TextStyle :: default () ;
                     the_struct . r#font_size = obj . 0 as _ ;
                     the_struct . r#font_weight = obj . 1 as _ ;
                     the_struct }
                ) . r#font_weight) as i32 }
            ) ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#i_text_input_18 }
             + r#TextInput :: FIELD_OFFSETS . r#height) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (((1f64 as f64) * (45f64 as f64)) as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#i_text_input_18 }
                 + r#TextInput :: FIELD_OFFSETS . r#read_only) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (false) as _ }
                ) ;
                 }
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#i_text_input_18 }
             + r#TextInput :: FIELD_OFFSETS . r#selection_background_color) . apply_pin (_self) . set ({
                 (slint :: Brush :: SolidColor (sp :: Color :: from_argb_encoded (4278221012f64 as u32)) . color ()) as sp :: Color }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#i_text_input_18 }
                 + r#TextInput :: FIELD_OFFSETS . r#selection_foreground_color) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#i_text_input_18 }
                     + r#TextInput :: FIELD_OFFSETS . r#color) . apply_pin (_self) . get () . color ()) as _ }
                ) ;
                 }
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#i_text_input_18 }
             + r#TextInput :: FIELD_OFFSETS . r#single_line) . apply_pin (_self) . set ({
                 (true) as bool }
            ) ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#i_text_input_18 }
             + r#TextInput :: FIELD_OFFSETS . r#text_cursor_width) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (1f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#i_text_input_18 }
             + r#TextInput :: FIELD_OFFSETS . r#vertical_alignment) . apply_pin (_self) . set ({
                 (sp :: r#TextVerticalAlignment :: r#Center) as sp :: r#TextVerticalAlignment }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#i_text_input_18 }
                 + r#TextInput :: FIELD_OFFSETS . r#width) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_i_layout_13_layout_cache }
                    ) . apply_pin (_self) . get () [1usize] as sp :: Coord) . max ((({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_i_text_input_18_preferred_width }
                    ) . apply_pin (_self) . get () . get () as sp :: Coord)) as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#i_text_input_18 }
                 + r#TextInput :: FIELD_OFFSETS . r#x) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((0f64 as sp :: Coord) . min (((((({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_i_layout_13_layout_cache }
                    ) . apply_pin (_self) . get () [1usize] as f64) - (({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#i_text_input_18 }
                     + r#TextInput :: FIELD_OFFSETS . r#width) . apply_pin (_self) . get () . get () as f64)) as sp :: Coord) . max ((({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_i_text_input_18_computed_x }
                    ) . apply_pin (_self) . get () . get () as sp :: Coord)) as sp :: Coord)) as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#i_focus_border_19 }
                 + r#Rectangle :: FIELD_OFFSETS . r#background) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (if ((({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_auth_input_11_state }
                    ) . apply_pin (_self) . get () as f64) == (2f64 as f64)) {
                         slint :: Brush :: SolidColor (if InnerColorSchemeSelector_37 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . root . get () . unwrap () . upgrade () . unwrap () . globals . global_ColorSchemeSelector_37 . as_ref ()) . get () {
                             sp :: Color :: from_argb_encoded (4284534271f64 as u32) }
                         else {
                             (sp :: Color :: from_argb_encoded (4278214584f64 as u32)) as _ }
                        ) }
                     else {
                         (slint :: Brush :: SolidColor (sp :: Color :: from_argb_encoded (0f64 as u32))) as _ }
                    ) as _ }
                ) ;
                 }
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#i_focus_border_19 }
             + r#Rectangle :: FIELD_OFFSETS . r#height) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (2f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#i_focus_border_19 }
                 + r#Rectangle :: FIELD_OFFSETS . r#width) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((({
                         let r#tmp_empty_2_padding = 40f64 ;
                         ;
                         ((((({
                             * & InnerMainWindow :: FIELD_OFFSETS . r#root_1 }
                         + r#WindowItem :: FIELD_OFFSETS . r#width) . apply_pin (_self) . get () . get () as f64) - (r#tmp_empty_2_padding . clone () as f64)) as f64) - (r#tmp_empty_2_padding . clone () as f64)) }
                     as f64) - (8f64 as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#i_focus_border_19 }
             + r#Rectangle :: FIELD_OFFSETS . r#x) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (4f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#i_focus_border_19 }
             + r#Rectangle :: FIELD_OFFSETS . r#y) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (43f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler (({
                     InnerMainWindow :: FIELD_OFFSETS . r#button_22 }
                 + {
                     * & InnerButton_root_25 :: FIELD_OFFSETS . r#root_25_clicked }
                ) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             ({
                                 * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_connect_clicked }
                            ) . apply_pin (_self) . call (& (({
                                 * & InnerMainWindow :: FIELD_OFFSETS . r#i_text_input_18 }
                             + r#TextInput :: FIELD_OFFSETS . r#text) . apply_pin (_self) . get () as _ ,) . into ()) }
                        ) ;
                         }
                     }
                ) ;
                 }
             ({
                 InnerMainWindow :: FIELD_OFFSETS . r#button_22 }
             + {
                 * & InnerButton_root_25 :: FIELD_OFFSETS . r#root_25 }
             + r#Empty :: FIELD_OFFSETS . r#height) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (45f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 InnerMainWindow :: FIELD_OFFSETS . r#button_22 }
             + {
                 * & InnerButton_root_25 :: FIELD_OFFSETS . r#root_25_primary }
            ) . apply_pin (_self) . set ({
                 (true) as bool }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerMainWindow :: FIELD_OFFSETS . r#button_22 }
                 + {
                     * & InnerButton_root_25 :: FIELD_OFFSETS . r#root_25_text }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (if ({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_is_connected }
                    ) . apply_pin (_self) . get () {
                         sp :: SharedString :: from ("断开连接") }
                     else {
                         (sp :: SharedString :: from ("一键接入")) as _ }
                    ) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerMainWindow :: FIELD_OFFSETS . r#button_22 }
                 + {
                     * & InnerButton_root_25 :: FIELD_OFFSETS . r#root_25 }
                 + r#Empty :: FIELD_OFFSETS . r#width) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ({
                         let r#tmp_empty_2_padding = 40f64 ;
                         ;
                         ((((({
                             * & InnerMainWindow :: FIELD_OFFSETS . r#root_1 }
                         + r#WindowItem :: FIELD_OFFSETS . r#width) . apply_pin (_self) . get () . get () as f64) - (r#tmp_empty_2_padding . clone () as f64)) as f64) - (r#tmp_empty_2_padding . clone () as f64)) }
                     as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 InnerMainWindow :: FIELD_OFFSETS . r#button_22 }
             + {
                 * & InnerButton_root_25 :: FIELD_OFFSETS . r#root_25 }
             + r#Empty :: FIELD_OFFSETS . r#x) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (40f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerMainWindow :: FIELD_OFFSETS . r#button_22 }
                 + {
                     * & InnerButton_root_25 :: FIELD_OFFSETS . r#root_25 }
                 + r#Empty :: FIELD_OFFSETS . r#y) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_empty_2_layout_cache }
                    ) . apply_pin (_self) . get () [8usize] as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#root_1 }
             + r#WindowItem :: FIELD_OFFSETS . r#always_on_top) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#root_1 }
             + r#WindowItem :: FIELD_OFFSETS . r#background) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#root_1 }
             + r#WindowItem :: FIELD_OFFSETS . r#default_font_family) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#root_1 }
             + r#WindowItem :: FIELD_OFFSETS . r#default_font_weight) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#root_1 }
             + r#WindowItem :: FIELD_OFFSETS . r#icon) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#root_1 }
             + r#WindowItem :: FIELD_OFFSETS . r#no_frame) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#root_1 }
             + r#WindowItem :: FIELD_OFFSETS . r#title) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#empty_3 }
             + r#Empty :: FIELD_OFFSETS . r#x) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#text_4 }
             + r#Text :: FIELD_OFFSETS . r#color) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#text_4 }
             + r#Text :: FIELD_OFFSETS . r#font_family) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#text_4 }
             + r#Text :: FIELD_OFFSETS . r#font_italic) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#text_4 }
             + r#Text :: FIELD_OFFSETS . r#font_size) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#text_4 }
             + r#Text :: FIELD_OFFSETS . r#font_weight) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#text_4 }
             + r#Text :: FIELD_OFFSETS . r#horizontal_alignment) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#text_4 }
             + r#Text :: FIELD_OFFSETS . r#letter_spacing) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#text_4 }
             + r#Text :: FIELD_OFFSETS . r#overflow) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#text_4 }
             + r#Text :: FIELD_OFFSETS . r#text) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#text_4 }
             + r#Text :: FIELD_OFFSETS . r#vertical_alignment) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#text_4 }
             + r#Text :: FIELD_OFFSETS . r#wrap) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#text_4 }
             + r#Text :: FIELD_OFFSETS . r#x) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#text_5 }
             + r#Text :: FIELD_OFFSETS . r#color) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#text_5 }
             + r#Text :: FIELD_OFFSETS . r#font_family) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#text_5 }
             + r#Text :: FIELD_OFFSETS . r#font_italic) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#text_5 }
             + r#Text :: FIELD_OFFSETS . r#font_size) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#text_5 }
             + r#Text :: FIELD_OFFSETS . r#font_weight) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#text_5 }
             + r#Text :: FIELD_OFFSETS . r#horizontal_alignment) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#text_5 }
             + r#Text :: FIELD_OFFSETS . r#letter_spacing) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#text_5 }
             + r#Text :: FIELD_OFFSETS . r#overflow) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#text_5 }
             + r#Text :: FIELD_OFFSETS . r#text) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#text_5 }
             + r#Text :: FIELD_OFFSETS . r#vertical_alignment) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#text_5 }
             + r#Text :: FIELD_OFFSETS . r#wrap) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#text_5 }
             + r#Text :: FIELD_OFFSETS . r#x) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#_shadow_6 }
             + r#BoxShadow :: FIELD_OFFSETS . r#border_radius) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#_shadow_6 }
             + r#BoxShadow :: FIELD_OFFSETS . r#height) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#_shadow_6 }
             + r#BoxShadow :: FIELD_OFFSETS . r#offset_x) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#_shadow_6 }
             + r#BoxShadow :: FIELD_OFFSETS . r#offset_y) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#_shadow_6 }
             + r#BoxShadow :: FIELD_OFFSETS . r#x) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#rectangle_7 }
             + r#BorderRectangle :: FIELD_OFFSETS . r#background) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#rectangle_7 }
             + r#BorderRectangle :: FIELD_OFFSETS . r#border_radius) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#rectangle_7 }
             + r#BorderRectangle :: FIELD_OFFSETS . r#border_width) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#rectangle_7 }
             + r#BorderRectangle :: FIELD_OFFSETS . r#height) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#rectangle_7 }
             + r#BorderRectangle :: FIELD_OFFSETS . r#x) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#text_9 }
             + r#Text :: FIELD_OFFSETS . r#font_family) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#text_9 }
             + r#Text :: FIELD_OFFSETS . r#font_italic) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#text_9 }
             + r#Text :: FIELD_OFFSETS . r#font_size) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#text_9 }
             + r#Text :: FIELD_OFFSETS . r#font_weight) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#text_9 }
             + r#Text :: FIELD_OFFSETS . r#horizontal_alignment) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#text_9 }
             + r#Text :: FIELD_OFFSETS . r#letter_spacing) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#text_9 }
             + r#Text :: FIELD_OFFSETS . r#overflow) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#text_9 }
             + r#Text :: FIELD_OFFSETS . r#vertical_alignment) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#text_9 }
             + r#Text :: FIELD_OFFSETS . r#wrap) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#text_9 }
             + r#Text :: FIELD_OFFSETS . r#x) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#text_10 }
             + r#Text :: FIELD_OFFSETS . r#color) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#text_10 }
             + r#Text :: FIELD_OFFSETS . r#font_family) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#text_10 }
             + r#Text :: FIELD_OFFSETS . r#font_italic) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#text_10 }
             + r#Text :: FIELD_OFFSETS . r#font_size) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#text_10 }
             + r#Text :: FIELD_OFFSETS . r#font_weight) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#text_10 }
             + r#Text :: FIELD_OFFSETS . r#horizontal_alignment) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#text_10 }
             + r#Text :: FIELD_OFFSETS . r#letter_spacing) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#text_10 }
             + r#Text :: FIELD_OFFSETS . r#overflow) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#text_10 }
             + r#Text :: FIELD_OFFSETS . r#vertical_alignment) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#text_10 }
             + r#Text :: FIELD_OFFSETS . r#wrap) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#text_10 }
             + r#Text :: FIELD_OFFSETS . r#x) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#auth_input_11 }
             + r#Empty :: FIELD_OFFSETS . r#height) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#auth_input_11 }
             + r#Empty :: FIELD_OFFSETS . r#x) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#i_background_12 }
             + r#BorderRectangle :: FIELD_OFFSETS . r#border_radius) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#i_background_12 }
             + r#BorderRectangle :: FIELD_OFFSETS . r#border_width) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#i_background_12 }
             + r#BorderRectangle :: FIELD_OFFSETS . r#height) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#i_background_12 }
             + r#BorderRectangle :: FIELD_OFFSETS . r#x) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#i_background_12 }
             + r#BorderRectangle :: FIELD_OFFSETS . r#y) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#rectangle_14 }
             + r#Empty :: FIELD_OFFSETS . r#height) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#rectangle_14 }
             + r#Empty :: FIELD_OFFSETS . r#y) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#_clip_15 }
             + r#Clip :: FIELD_OFFSETS . r#border_radius) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#_clip_15 }
             + r#Clip :: FIELD_OFFSETS . r#border_width) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#_clip_15 }
             + r#Clip :: FIELD_OFFSETS . r#height) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#_clip_15 }
             + r#Clip :: FIELD_OFFSETS . r#x) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#_clip_15 }
             + r#Clip :: FIELD_OFFSETS . r#y) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#i_placeholder_visibility_16 }
             + r#Clip :: FIELD_OFFSETS . r#border_radius) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#i_placeholder_visibility_16 }
             + r#Clip :: FIELD_OFFSETS . r#border_width) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#i_placeholder_visibility_16 }
             + r#Clip :: FIELD_OFFSETS . r#height) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#i_placeholder_visibility_16 }
             + r#Clip :: FIELD_OFFSETS . r#width) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#i_placeholder_visibility_16 }
             + r#Clip :: FIELD_OFFSETS . r#x) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#i_placeholder_visibility_16 }
             + r#Clip :: FIELD_OFFSETS . r#y) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#i_placeholder_17 }
             + r#Text :: FIELD_OFFSETS . r#font_family) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#i_placeholder_17 }
             + r#Text :: FIELD_OFFSETS . r#font_italic) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#i_placeholder_17 }
             + r#Text :: FIELD_OFFSETS . r#font_weight) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#i_placeholder_17 }
             + r#Text :: FIELD_OFFSETS . r#height) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#i_placeholder_17 }
             + r#Text :: FIELD_OFFSETS . r#horizontal_alignment) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#i_placeholder_17 }
             + r#Text :: FIELD_OFFSETS . r#letter_spacing) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#i_placeholder_17 }
             + r#Text :: FIELD_OFFSETS . r#overflow) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#i_placeholder_17 }
             + r#Text :: FIELD_OFFSETS . r#text) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#i_placeholder_17 }
             + r#Text :: FIELD_OFFSETS . r#vertical_alignment) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#i_placeholder_17 }
             + r#Text :: FIELD_OFFSETS . r#wrap) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#i_placeholder_17 }
             + r#Text :: FIELD_OFFSETS . r#x) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#i_placeholder_17 }
             + r#Text :: FIELD_OFFSETS . r#y) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#i_text_input_18 }
             + r#TextInput :: FIELD_OFFSETS . r#font_family) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#i_text_input_18 }
             + r#TextInput :: FIELD_OFFSETS . r#font_italic) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#i_text_input_18 }
             + r#TextInput :: FIELD_OFFSETS . r#font_weight) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#i_text_input_18 }
             + r#TextInput :: FIELD_OFFSETS . r#height) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#i_text_input_18 }
             + r#TextInput :: FIELD_OFFSETS . r#horizontal_alignment) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#i_text_input_18 }
             + r#TextInput :: FIELD_OFFSETS . r#input_type) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#i_text_input_18 }
             + r#TextInput :: FIELD_OFFSETS . r#letter_spacing) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#i_text_input_18 }
             + r#TextInput :: FIELD_OFFSETS . r#selection_background_color) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#i_text_input_18 }
             + r#TextInput :: FIELD_OFFSETS . r#single_line) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#i_text_input_18 }
             + r#TextInput :: FIELD_OFFSETS . r#text_cursor_width) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#i_text_input_18 }
             + r#TextInput :: FIELD_OFFSETS . r#vertical_alignment) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#i_text_input_18 }
             + r#TextInput :: FIELD_OFFSETS . r#wrap) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#i_text_input_18 }
             + r#TextInput :: FIELD_OFFSETS . r#y) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#i_focus_border_19 }
             + r#Rectangle :: FIELD_OFFSETS . r#height) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#i_focus_border_19 }
             + r#Rectangle :: FIELD_OFFSETS . r#x) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#i_focus_border_19 }
             + r#Rectangle :: FIELD_OFFSETS . r#y) . apply_pin (_self) . set_constant () ;
             ({
                 InnerMainWindow :: FIELD_OFFSETS . r#button_22 }
             + {
                 * & InnerButton_root_25 :: FIELD_OFFSETS . r#root_25_checkable }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerMainWindow :: FIELD_OFFSETS . r#button_22 }
             + {
                 * & InnerButton_root_25 :: FIELD_OFFSETS . r#root_25 }
             + r#Empty :: FIELD_OFFSETS . r#height) . apply_pin (_self) . set_constant () ;
             ({
                 InnerMainWindow :: FIELD_OFFSETS . r#button_22 }
             + {
                 * & InnerButton_root_25 :: FIELD_OFFSETS . r#root_25_icon }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerMainWindow :: FIELD_OFFSETS . r#button_22 }
             + {
                 * & InnerButton_root_25 :: FIELD_OFFSETS . r#root_25_primary }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerMainWindow :: FIELD_OFFSETS . r#button_22 }
             + {
                 * & InnerButton_root_25 :: FIELD_OFFSETS . r#root_25 }
             + r#Empty :: FIELD_OFFSETS . r#x) . apply_pin (_self) . set_constant () ;
             }
         pub fn user_init (self_rc : sp :: VRcMapped < sp :: ComponentVTable , Self >) {
             let _self = self_rc . as_pin_ref () ;
             InnerButton_root_25 :: user_init (VRcMapped :: map (self_rc . clone () , | x | {
                 * & Self :: FIELD_OFFSETS . r#button_22 }
             . apply_pin (x)) ,) ;
             {
                 }
             ;
             {
                 }
             ;
             {
                 }
             ;
             {
                 }
             ;
             }
         fn visit_dynamic_children (self : :: core :: pin :: Pin < & Self > , dyn_index : usize , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut) -> sp :: VisitChildrenResult {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 0usize => {
                     InnerMainWindow :: FIELD_OFFSETS . repeater0 . apply_pin (_self) . ensure_updated (|| InnerComponent_text_20 :: new (_self . self_weak . get () . unwrap () . clone ()) . into ()) ;
                     _self . repeater0 . visit (order , visitor) }
                 1usize ..= 3usize => {
                     {
                         * & Self :: FIELD_OFFSETS . r#button_22 }
                     . apply_pin (_self) . visit_dynamic_children (dyn_index - 1usize , order , visitor) }
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn layout_info (self : :: core :: pin :: Pin < & Self > , orientation : sp :: Orientation) -> sp :: LayoutInfo {
             # ! [allow (unused)] let _self = self ;
             match orientation {
                 sp :: Orientation :: Horizontal => {
                     let r#layout_info = ({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_layoutinfo_h }
                    ) . apply_pin (_self) . get () ;
                     ;
                     {
                         let mut the_struct = r#LayoutInfo :: default () ;
                         the_struct . r#max = ({
                             * & InnerMainWindow :: FIELD_OFFSETS . r#root_1 }
                         + r#WindowItem :: FIELD_OFFSETS . r#width) . apply_pin (_self) . get () . get () as _ ;
                         the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                         the_struct . r#min = ({
                             * & InnerMainWindow :: FIELD_OFFSETS . r#root_1 }
                         + r#WindowItem :: FIELD_OFFSETS . r#width) . apply_pin (_self) . get () . get () as _ ;
                         the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                         the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                         the_struct . r#stretch = (r#layout_info . clone ()) . r#stretch as _ ;
                         the_struct }
                     }
                 , sp :: Orientation :: Vertical => {
                     let r#layout_info = ({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_layoutinfo_v }
                    ) . apply_pin (_self) . get () ;
                     ;
                     {
                         let mut the_struct = r#LayoutInfo :: default () ;
                         the_struct . r#max = ({
                             * & InnerMainWindow :: FIELD_OFFSETS . r#root_1 }
                         + r#WindowItem :: FIELD_OFFSETS . r#height) . apply_pin (_self) . get () . get () as _ ;
                         the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                         the_struct . r#min = ({
                             * & InnerMainWindow :: FIELD_OFFSETS . r#root_1 }
                         + r#WindowItem :: FIELD_OFFSETS . r#height) . apply_pin (_self) . get () . get () as _ ;
                         the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                         the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                         the_struct . r#stretch = (r#layout_info . clone ()) . r#stretch as _ ;
                         the_struct }
                     }
                 , }
             }
         fn subtree_range (self : :: core :: pin :: Pin < & Self > , dyn_index : usize) -> sp :: IndexRange {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 0usize => {
                     InnerMainWindow :: FIELD_OFFSETS . repeater0 . apply_pin (_self) . ensure_updated (|| InnerComponent_text_20 :: new (_self . self_weak . get () . unwrap () . clone ()) . into ()) ;
                     sp :: IndexRange :: from (_self . repeater0 . range ()) }
                 1usize ..= 3usize => {
                     {
                         * & Self :: FIELD_OFFSETS . r#button_22 }
                     . apply_pin (_self) . subtree_range (dyn_index - 1usize) }
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn subtree_component (self : :: core :: pin :: Pin < & Self > , dyn_index : usize , subtree_index : usize , result : & mut sp :: ComponentWeak) {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 0usize => {
                     InnerMainWindow :: FIELD_OFFSETS . repeater0 . apply_pin (_self) . ensure_updated (|| InnerComponent_text_20 :: new (_self . self_weak . get () . unwrap () . clone ()) . into ()) ;
                     * result = vtable :: VRc :: downgrade (& vtable :: VRc :: into_dyn (_self . repeater0 . component_at (subtree_index) . unwrap ())) }
                 1usize ..= 3usize => {
                     {
                         * & Self :: FIELD_OFFSETS . r#button_22 }
                     . apply_pin (_self) . subtree_component (dyn_index - 1usize , subtree_index , result) }
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             ;
             }
         fn index_property (self : :: core :: pin :: Pin < & Self >) -> usize {
             # ! [allow (unused)] let _self = self ;
             core :: usize :: MAX }
         fn accessible_role (self : :: core :: pin :: Pin < & Self > , index : usize) -> sp :: AccessibleRole {
             # ! [allow (unused)] let _self = self ;
             match index {
                 6usize => sp :: r#AccessibleRole :: r#Button , 7usize => sp :: r#AccessibleRole :: r#Text , 8usize => sp :: r#AccessibleRole :: r#Text , 9usize => sp :: r#AccessibleRole :: r#Text , 10usize => sp :: r#AccessibleRole :: r#Text , 17usize => sp :: r#AccessibleRole :: r#Text , 6usize => {
                     * & Self :: FIELD_OFFSETS . r#button_22 }
                 . apply_pin (_self) . accessible_role (0) , 18usize ..= 21usize => {
                     * & Self :: FIELD_OFFSETS . r#button_22 }
                 . apply_pin (_self) . accessible_role (index - 18usize + 1) , _ => AccessibleRole :: default () , }
             }
         fn accessible_string_property (self : :: core :: pin :: Pin < & Self > , index : usize , what : sp :: AccessibleStringProperty ,) -> sp :: SharedString {
             # ! [allow (unused)] let _self = self ;
             match (index , what) {
                 (6usize , AccessibleStringProperty :: r#Label) => ({
                     InnerMainWindow :: FIELD_OFFSETS . r#button_22 }
                 + {
                     * & InnerButton_root_25 :: FIELD_OFFSETS . r#root_25_text }
                ) . apply_pin (_self) . get () , (7usize , AccessibleStringProperty :: r#Label) => sp :: SharedString :: from ("Subnetra") , (8usize , AccessibleStringProperty :: r#Label) => sp :: SharedString :: from ("企业级虚拟安全网络") , (9usize , AccessibleStringProperty :: r#Label) => ({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#text_9 }
                 + r#Text :: FIELD_OFFSETS . r#text) . apply_pin (_self) . get () , (10usize , AccessibleStringProperty :: r#Label) => ({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#text_10 }
                 + r#Text :: FIELD_OFFSETS . r#text) . apply_pin (_self) . get () , (17usize , AccessibleStringProperty :: r#Label) => sp :: SharedString :: from ("粘贴邀请码 (Auth Key)") , (6usize , _) => {
                     * & Self :: FIELD_OFFSETS . r#button_22 }
                 . apply_pin (_self) . accessible_string_property (0 , what) , (18usize ..= 21usize , _) => {
                     * & Self :: FIELD_OFFSETS . r#button_22 }
                 . apply_pin (_self) . accessible_string_property (index - 18usize + 1 , what) , _ => Default :: default () , }
             }
         }
     # [derive (sp :: FieldOffsets , Default)] # [const_field_offset (sp :: const_field_offset)] # [repr (C)] # [pin_drop] struct InnerComponent_text_20 {
         r#text_20 : sp :: r#Text , self_weak : sp :: OnceCell < sp :: VWeakMapped < sp :: ComponentVTable , InnerComponent_text_20 >> , parent : sp :: VWeakMapped :: < sp :: ComponentVTable , InnerMainWindow > , root : sp :: OnceCell < sp :: VWeak < sp :: ComponentVTable , InnerMainWindow >> , tree_index : :: core :: cell :: Cell < u32 > , tree_index_of_first_child : :: core :: cell :: Cell < u32 > , }
     impl InnerComponent_text_20 {
         pub fn init (self_rc : sp :: VRcMapped < sp :: ComponentVTable , Self > , root : & sp :: VRc < sp :: ComponentVTable , InnerMainWindow > , tree_index : u32 , tree_index_of_first_child : u32) {
             # ! [allow (unused)] let _self = self_rc . as_pin_ref () ;
             _self . self_weak . set (VRcMapped :: downgrade (& self_rc)) ;
             _self . root . set (VRc :: downgrade (root)) ;
             _self . tree_index . set (tree_index) ;
             _self . tree_index_of_first_child . set (tree_index_of_first_child) ;
             ({
                 * & InnerComponent_text_20 :: FIELD_OFFSETS . r#text_20 }
             + r#Text :: FIELD_OFFSETS . r#color) . apply_pin (_self) . set ({
                 (slint :: Brush :: SolidColor (sp :: Color :: from_argb_encoded (4293870660f64 as u32))) as slint :: Brush }
            ) ;
             ({
                 * & InnerComponent_text_20 :: FIELD_OFFSETS . r#text_20 }
             + r#Text :: FIELD_OFFSETS . r#font_size) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (12f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_text_20 :: FIELD_OFFSETS . r#text_20 }
                 + r#Text :: FIELD_OFFSETS . r#height) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ({
                         let cache = (InnerMainWindow :: FIELD_OFFSETS . r#root_1_empty_2_layout_cache) . apply_pin (_self . parent . upgrade () . unwrap () . as_pin_ref ()) . get () ;
                         * cache . get ((cache [7usize] as usize) + 0f64 as usize * 2) . unwrap_or (& (0 as sp :: Coord)) }
                     as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & InnerComponent_text_20 :: FIELD_OFFSETS . r#text_20 }
             + r#Text :: FIELD_OFFSETS . r#horizontal_alignment) . apply_pin (_self) . set ({
                 (sp :: r#TextHorizontalAlignment :: r#Center) as sp :: r#TextHorizontalAlignment }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_text_20 :: FIELD_OFFSETS . r#text_20 }
                 + r#Text :: FIELD_OFFSETS . r#text) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ((InnerMainWindow :: FIELD_OFFSETS . r#root_1_error_message) . apply_pin (_self . parent . upgrade () . unwrap () . as_pin_ref ()) . get ()) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_text_20 :: FIELD_OFFSETS . r#text_20 }
                 + r#Text :: FIELD_OFFSETS . r#width) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ({
                         let r#tmp_empty_2_padding = 40f64 ;
                         ;
                         ((((({
                             * & InnerMainWindow :: FIELD_OFFSETS . r#root_1 }
                         + r#WindowItem :: FIELD_OFFSETS . r#width) . apply_pin (_self . parent . upgrade () . unwrap () . as_pin_ref ()) . get () . get () as f64) - (r#tmp_empty_2_padding . clone () as f64)) as f64) - (r#tmp_empty_2_padding . clone () as f64)) }
                     as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & InnerComponent_text_20 :: FIELD_OFFSETS . r#text_20 }
             + r#Text :: FIELD_OFFSETS . r#wrap) . apply_pin (_self) . set ({
                 (sp :: r#TextWrap :: r#WordWrap) as sp :: r#TextWrap }
            ) ;
             ({
                 * & InnerComponent_text_20 :: FIELD_OFFSETS . r#text_20 }
             + r#Text :: FIELD_OFFSETS . r#x) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (40f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_text_20 :: FIELD_OFFSETS . r#text_20 }
                 + r#Text :: FIELD_OFFSETS . r#y) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ({
                         let cache = (InnerMainWindow :: FIELD_OFFSETS . r#root_1_empty_2_layout_cache) . apply_pin (_self . parent . upgrade () . unwrap () . as_pin_ref ()) . get () ;
                         * cache . get ((cache [6usize] as usize) + 0f64 as usize * 2) . unwrap_or (& (0 as sp :: Coord)) }
                     as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & InnerComponent_text_20 :: FIELD_OFFSETS . r#text_20 }
             + r#Text :: FIELD_OFFSETS . r#color) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_text_20 :: FIELD_OFFSETS . r#text_20 }
             + r#Text :: FIELD_OFFSETS . r#font_family) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_text_20 :: FIELD_OFFSETS . r#text_20 }
             + r#Text :: FIELD_OFFSETS . r#font_italic) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_text_20 :: FIELD_OFFSETS . r#text_20 }
             + r#Text :: FIELD_OFFSETS . r#font_size) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_text_20 :: FIELD_OFFSETS . r#text_20 }
             + r#Text :: FIELD_OFFSETS . r#font_weight) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_text_20 :: FIELD_OFFSETS . r#text_20 }
             + r#Text :: FIELD_OFFSETS . r#horizontal_alignment) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_text_20 :: FIELD_OFFSETS . r#text_20 }
             + r#Text :: FIELD_OFFSETS . r#letter_spacing) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_text_20 :: FIELD_OFFSETS . r#text_20 }
             + r#Text :: FIELD_OFFSETS . r#overflow) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_text_20 :: FIELD_OFFSETS . r#text_20 }
             + r#Text :: FIELD_OFFSETS . r#vertical_alignment) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_text_20 :: FIELD_OFFSETS . r#text_20 }
             + r#Text :: FIELD_OFFSETS . r#wrap) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_text_20 :: FIELD_OFFSETS . r#text_20 }
             + r#Text :: FIELD_OFFSETS . r#x) . apply_pin (_self) . set_constant () ;
             }
         pub fn user_init (self_rc : sp :: VRcMapped < sp :: ComponentVTable , Self >) {
             let _self = self_rc . as_pin_ref () ;
             }
         fn visit_dynamic_children (self : :: core :: pin :: Pin < & Self > , dyn_index : usize , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut) -> sp :: VisitChildrenResult {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn layout_info (self : :: core :: pin :: Pin < & Self > , orientation : sp :: Orientation) -> sp :: LayoutInfo {
             # ! [allow (unused)] let _self = self ;
             match orientation {
                 sp :: Orientation :: Horizontal => sp :: Item :: layout_info (({
                     * & InnerComponent_text_20 :: FIELD_OFFSETS . r#text_20 }
                ) . apply_pin (_self) , sp :: Orientation :: Horizontal , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ())) , sp :: Orientation :: Vertical => sp :: Item :: layout_info (({
                     * & InnerComponent_text_20 :: FIELD_OFFSETS . r#text_20 }
                ) . apply_pin (_self) , sp :: Orientation :: Vertical , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ())) , }
             }
         fn subtree_range (self : :: core :: pin :: Pin < & Self > , dyn_index : usize) -> sp :: IndexRange {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn subtree_component (self : :: core :: pin :: Pin < & Self > , dyn_index : usize , subtree_index : usize , result : & mut sp :: ComponentWeak) {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             ;
             }
         fn index_property (self : :: core :: pin :: Pin < & Self >) -> usize {
             # ! [allow (unused)] let _self = self ;
             core :: usize :: MAX }
         fn accessible_role (self : :: core :: pin :: Pin < & Self > , index : usize) -> sp :: AccessibleRole {
             # ! [allow (unused)] let _self = self ;
             match index {
                 0usize => sp :: r#AccessibleRole :: r#Text , _ => AccessibleRole :: default () , }
             }
         fn accessible_string_property (self : :: core :: pin :: Pin < & Self > , index : usize , what : sp :: AccessibleStringProperty ,) -> sp :: SharedString {
             # ! [allow (unused)] let _self = self ;
             match (index , what) {
                 (0usize , AccessibleStringProperty :: r#Label) => (InnerMainWindow :: FIELD_OFFSETS . r#root_1_error_message) . apply_pin (_self . parent . upgrade () . unwrap () . as_pin_ref ()) . get () , _ => Default :: default () , }
             }
         }
     impl InnerComponent_text_20 {
         pub fn new (parent : sp :: VWeakMapped :: < sp :: ComponentVTable , InnerMainWindow >) -> vtable :: VRc < sp :: ComponentVTable , Self > {
             # ! [allow (unused)] slint :: private_unstable_api :: ensure_backend () ;
             let mut _self = Self :: default () ;
             _self . parent = parent . clone () as sp :: VWeakMapped :: < sp :: ComponentVTable , InnerMainWindow > ;
             let self_rc = VRc :: new (_self) ;
             let self_dyn_rc = vtable :: VRc :: into_dyn (self_rc . clone ()) ;
             sp :: register_component (& self_dyn_rc , (* & parent . upgrade () . unwrap () . root . get () . unwrap () . upgrade () . unwrap ()) . maybe_window_adapter_impl ()) ;
             Self :: init (sp :: VRc :: map (self_rc . clone () , | x | x) , & parent . upgrade () . unwrap () . root . get () . unwrap () . upgrade () . unwrap () , 0 , 1) ;
             self_rc }
         fn item_tree () -> & 'static [sp :: ItemTreeNode] {
             const ITEM_TREE : [sp :: ItemTreeNode ;
             1usize] = [sp :: ItemTreeNode :: Item {
                 is_accessible : true , children_count : 0u32 , children_index : 1u32 , parent_index : 0u32 , item_array_index : 0u32 , }
            ] ;
             & ITEM_TREE }
         fn item_array () -> & 'static [vtable :: VOffset < Self , ItemVTable , vtable :: AllowPin >] {
             static ITEM_ARRAY : sp :: OnceBox < [vtable :: VOffset < InnerComponent_text_20 , ItemVTable , vtable :: AllowPin > ;
             1usize] > = sp :: OnceBox :: new () ;
             & * ITEM_ARRAY . get_or_init (|| Box :: new ([VOffset :: new ({
                 * & InnerComponent_text_20 :: FIELD_OFFSETS . r#text_20 }
            )])) }
         # [allow (unused)] fn window_adapter_impl (& self) -> Rc < dyn sp :: WindowAdapter > {
             self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl () }
         # [allow (unused)] fn maybe_window_adapter_impl (& self) -> Option < Rc < dyn sp :: WindowAdapter >> {
             self . root . get () . and_then (| root_weak | root_weak . upgrade ()) . and_then (| root | root . maybe_window_adapter_impl ()) }
         }
     impl sp :: PinnedDrop for InnerComponent_text_20 {
         fn drop (self : core :: pin :: Pin < & mut InnerComponent_text_20 >) {
             use slint :: private_unstable_api :: re_exports :: * ;
             ComponentVTable_static ! (static VT for self :: InnerComponent_text_20) ;
             new_vref ! (let vref : VRef < ComponentVTable > for Component = self . as_ref () . get_ref ()) ;
             if let Some (wa) = self . maybe_window_adapter_impl () {
                 sp :: unregister_component (self . as_ref () , vref , Self :: item_array () , & wa) ;
                 }
             }
         }
     impl sp :: Component for InnerComponent_text_20 {
         fn visit_children_item (self : :: core :: pin :: Pin < & Self > , index : isize , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut) -> sp :: VisitChildrenResult {
             return sp :: visit_item_tree (self , & VRcMapped :: origin (& self . as_ref () . self_weak . get () . unwrap () . upgrade () . unwrap ()) , self . get_item_tree () . as_slice () , index , order , visitor , visit_dynamic) ;
             # [allow (unused)] fn visit_dynamic (_self : :: core :: pin :: Pin < & InnerComponent_text_20 > , order : sp :: TraversalOrder , visitor : ItemVisitorRefMut , dyn_index : usize) -> VisitChildrenResult {
                 _self . visit_dynamic_children (dyn_index , order , visitor) }
             }
         fn get_item_ref (self : :: core :: pin :: Pin < & Self > , index : usize) -> :: core :: pin :: Pin < ItemRef > {
             match & self . get_item_tree () . as_slice () [index] {
                 ItemTreeNode :: Item {
                     item_array_index , .. }
                 => {
                     Self :: item_array () [* item_array_index as usize] . apply_pin (self) }
                 ItemTreeNode :: DynamicTree {
                     .. }
                 => panic ! ("get_item_ref called on dynamic tree") , }
             }
         fn get_item_tree (self : :: core :: pin :: Pin < & Self >) -> sp :: Slice < sp :: ItemTreeNode > {
             Self :: item_tree () . into () }
         fn get_subtree_range (self : :: core :: pin :: Pin < & Self > , index : usize) -> sp :: IndexRange {
             self . subtree_range (index) }
         fn get_subtree_component (self : :: core :: pin :: Pin < & Self > , index : usize , subtree_index : usize , result : & mut sp :: ComponentWeak) {
             self . subtree_component (index , subtree_index , result) ;
             }
         fn subtree_index (self : :: core :: pin :: Pin < & Self >) -> usize {
             self . index_property () }
         fn parent_node (self : :: core :: pin :: Pin < & Self > , _result : & mut sp :: ItemWeak) {
             if let Some ((parent_component , parent_index)) = self . parent . clone () . upgrade () . map (| sc | (VRcMapped :: origin (& sc) , sc . tree_index_of_first_child . get ())) {
                 * _result = sp :: ItemRc :: new (parent_component , parent_index as usize + 5usize - 1) . downgrade () ;
                 }
             }
         fn embed_component (self : :: core :: pin :: Pin < & Self > , _parent_component : & sp :: ComponentWeak , _item_tree_index : usize) -> bool {
             todo ! ("Components written in Rust can not get embedded yet.") }
         fn layout_info (self : :: core :: pin :: Pin < & Self > , orientation : sp :: Orientation) -> sp :: LayoutInfo {
             self . layout_info (orientation) }
         fn accessible_role (self : :: core :: pin :: Pin < & Self > , index : usize) -> sp :: AccessibleRole {
             self . accessible_role (index) }
         fn accessible_string_property (self : :: core :: pin :: Pin < & Self > , index : usize , what : sp :: AccessibleStringProperty , result : & mut sp :: SharedString ,) {
             * result = self . accessible_string_property (index , what) ;
             }
         fn window_adapter (self : :: core :: pin :: Pin < & Self > , do_create : bool , result : & mut Option < Rc < dyn WindowAdapter >> ,) {
             if do_create {
                 * result = Some (self . window_adapter_impl ()) ;
                 }
             else {
                 * result = self . maybe_window_adapter_impl () ;
                 }
             }
         }
     impl sp :: RepeatedComponent for InnerComponent_text_20 {
         type Data = () ;
         fn update (& self , _index : usize , _data : Self :: Data) {
             let self_rc = self . self_weak . get () . unwrap () . upgrade () . unwrap () ;
             let _self = self_rc . as_pin_ref () ;
             }
         fn init (& self) {
             let self_rc = self . self_weak . get () . unwrap () . upgrade () . unwrap () ;
             InnerComponent_text_20 :: user_init (VRcMapped :: map (self_rc , | x | x) ,) ;
             }
         fn box_layout_data (self : :: core :: pin :: Pin < & Self > , o : sp :: Orientation) -> sp :: BoxLayoutCellData {
             BoxLayoutCellData {
                 constraint : self . as_ref () . layout_info (o) }
             }
         }
     impl InnerMainWindow {
         pub fn new () -> core :: result :: Result < vtable :: VRc < sp :: ComponentVTable , Self > , slint :: PlatformError > {
             # ! [allow (unused)] slint :: private_unstable_api :: ensure_backend () ;
             let mut _self = Self :: default () ;
             let self_rc = VRc :: new (_self) ;
             let self_dyn_rc = vtable :: VRc :: into_dyn (self_rc . clone ()) ;
             sp :: register_component (& self_dyn_rc , (* & self_rc) . maybe_window_adapter_impl ()) ;
             Self :: init (sp :: VRc :: map (self_rc . clone () , | x | x) , & self_rc , 0 , 1) ;
             core :: result :: Result :: Ok (self_rc) }
         fn item_tree () -> & 'static [sp :: ItemTreeNode] {
             const ITEM_TREE : [sp :: ItemTreeNode ;
             25usize] = [sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 6u32 , children_index : 1u32 , parent_index : 0u32 , item_array_index : 0u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 2u32 , children_index : 7u32 , parent_index : 0u32 , item_array_index : 1u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 9u32 , parent_index : 0u32 , item_array_index : 2u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 2u32 , children_index : 9u32 , parent_index : 0u32 , item_array_index : 3u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 11u32 , parent_index : 0u32 , item_array_index : 4u32 , }
             , sp :: ItemTreeNode :: DynamicTree {
                 index : 0usize , parent_index : 0u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : true , children_count : 4u32 , children_index : 18u32 , parent_index : 0u32 , item_array_index : 5u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : true , children_count : 0u32 , children_index : 9u32 , parent_index : 1u32 , item_array_index : 6u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : true , children_count : 0u32 , children_index : 9u32 , parent_index : 1u32 , item_array_index : 7u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : true , children_count : 0u32 , children_index : 11u32 , parent_index : 3u32 , item_array_index : 8u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : true , children_count : 0u32 , children_index : 11u32 , parent_index : 3u32 , item_array_index : 9u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 2u32 , children_index : 12u32 , parent_index : 4u32 , item_array_index : 10u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 14u32 , parent_index : 11u32 , item_array_index : 11u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 18u32 , parent_index : 11u32 , item_array_index : 12u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 2u32 , children_index : 15u32 , parent_index : 12u32 , item_array_index : 13u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 17u32 , parent_index : 14u32 , item_array_index : 14u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 18u32 , parent_index : 14u32 , item_array_index : 15u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : true , children_count : 0u32 , children_index : 18u32 , parent_index : 15u32 , item_array_index : 16u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 3u32 , children_index : 22u32 , parent_index : 6u32 , item_array_index : 17u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 25u32 , parent_index : 6u32 , item_array_index : 18u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 25u32 , parent_index : 6u32 , item_array_index : 19u32 , }
             , sp :: ItemTreeNode :: DynamicTree {
                 index : 3usize , parent_index : 6u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 25u32 , parent_index : 18u32 , item_array_index : 20u32 , }
             , sp :: ItemTreeNode :: DynamicTree {
                 index : 1usize , parent_index : 18u32 , }
             , sp :: ItemTreeNode :: DynamicTree {
                 index : 2usize , parent_index : 18u32 , }
            ] ;
             & ITEM_TREE }
         fn item_array () -> & 'static [vtable :: VOffset < Self , ItemVTable , vtable :: AllowPin >] {
             static ITEM_ARRAY : sp :: OnceBox < [vtable :: VOffset < InnerMainWindow , ItemVTable , vtable :: AllowPin > ;
             21usize] > = sp :: OnceBox :: new () ;
             & * ITEM_ARRAY . get_or_init (|| Box :: new ([VOffset :: new ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#root_1 }
            ) , VOffset :: new ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#empty_3 }
            ) , VOffset :: new ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#_shadow_6 }
            ) , VOffset :: new ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#rectangle_7 }
            ) , VOffset :: new ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#auth_input_11 }
            ) , VOffset :: new ({
                 InnerMainWindow :: FIELD_OFFSETS . r#button_22 }
             + {
                 * & InnerButton_root_25 :: FIELD_OFFSETS . r#root_25 }
            ) , VOffset :: new ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#text_4 }
            ) , VOffset :: new ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#text_5 }
            ) , VOffset :: new ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#text_9 }
            ) , VOffset :: new ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#text_10 }
            ) , VOffset :: new ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#i_background_12 }
            ) , VOffset :: new ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#rectangle_14 }
            ) , VOffset :: new ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#i_focus_border_19 }
            ) , VOffset :: new ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#_clip_15 }
            ) , VOffset :: new ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#i_placeholder_visibility_16 }
            ) , VOffset :: new ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#i_text_input_18 }
            ) , VOffset :: new ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#i_placeholder_17 }
            ) , VOffset :: new ({
                 InnerMainWindow :: FIELD_OFFSETS . r#button_22 }
             + {
                 * & InnerButton_root_25 :: FIELD_OFFSETS . r#i_background_26 }
            ) , VOffset :: new ({
                 InnerMainWindow :: FIELD_OFFSETS . r#button_22 }
             + {
                 * & InnerButton_root_25 :: FIELD_OFFSETS . r#i_touch_area_33 }
            ) , VOffset :: new ({
                 InnerMainWindow :: FIELD_OFFSETS . r#button_22 }
             + {
                 * & InnerButton_root_25 :: FIELD_OFFSETS . r#i_focus_scope_34 }
            ) , VOffset :: new ({
                 InnerMainWindow :: FIELD_OFFSETS . r#button_22 }
             + {
                 * & InnerButton_root_25 :: FIELD_OFFSETS . r#i_border_27 }
            )])) }
         # [allow (unused)] fn window_adapter_impl (& self) -> Rc < dyn sp :: WindowAdapter > {
             Rc :: clone (self . window_adapter_ref () . unwrap ()) }
         fn window_adapter_ref (& self ,) -> Result < & Rc < dyn sp :: WindowAdapter > , slint :: PlatformError > {
             self . window_adapter_ . get_or_try_init (|| {
                 let adapter = slint :: private_unstable_api :: create_window_adapter () ? ;
                 let self_rc = VRcMapped :: origin (& self . self_weak . get () . unwrap () . upgrade () . unwrap ()) ;
                 sp :: WindowInner :: from_pub (adapter . window ()) . set_component (& self_rc) ;
                 core :: result :: Result :: Ok (adapter) }
            ) }
         # [allow (unused)] fn maybe_window_adapter_impl (& self) -> Option < Rc < dyn sp :: WindowAdapter >> {
             self . window_adapter_ . get () . cloned () }
         }
     impl sp :: PinnedDrop for InnerMainWindow {
         fn drop (self : core :: pin :: Pin < & mut InnerMainWindow >) {
             use slint :: private_unstable_api :: re_exports :: * ;
             ComponentVTable_static ! (static VT for self :: InnerMainWindow) ;
             new_vref ! (let vref : VRef < ComponentVTable > for Component = self . as_ref () . get_ref ()) ;
             if let Some (wa) = self . maybe_window_adapter_impl () {
                 sp :: unregister_component (self . as_ref () , vref , Self :: item_array () , & wa) ;
                 }
             }
         }
     impl sp :: Component for InnerMainWindow {
         fn visit_children_item (self : :: core :: pin :: Pin < & Self > , index : isize , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut) -> sp :: VisitChildrenResult {
             return sp :: visit_item_tree (self , & VRcMapped :: origin (& self . as_ref () . self_weak . get () . unwrap () . upgrade () . unwrap ()) , self . get_item_tree () . as_slice () , index , order , visitor , visit_dynamic) ;
             # [allow (unused)] fn visit_dynamic (_self : :: core :: pin :: Pin < & InnerMainWindow > , order : sp :: TraversalOrder , visitor : ItemVisitorRefMut , dyn_index : usize) -> VisitChildrenResult {
                 _self . visit_dynamic_children (dyn_index , order , visitor) }
             }
         fn get_item_ref (self : :: core :: pin :: Pin < & Self > , index : usize) -> :: core :: pin :: Pin < ItemRef > {
             match & self . get_item_tree () . as_slice () [index] {
                 ItemTreeNode :: Item {
                     item_array_index , .. }
                 => {
                     Self :: item_array () [* item_array_index as usize] . apply_pin (self) }
                 ItemTreeNode :: DynamicTree {
                     .. }
                 => panic ! ("get_item_ref called on dynamic tree") , }
             }
         fn get_item_tree (self : :: core :: pin :: Pin < & Self >) -> sp :: Slice < sp :: ItemTreeNode > {
             Self :: item_tree () . into () }
         fn get_subtree_range (self : :: core :: pin :: Pin < & Self > , index : usize) -> sp :: IndexRange {
             self . subtree_range (index) }
         fn get_subtree_component (self : :: core :: pin :: Pin < & Self > , index : usize , subtree_index : usize , result : & mut sp :: ComponentWeak) {
             self . subtree_component (index , subtree_index , result) ;
             }
         fn subtree_index (self : :: core :: pin :: Pin < & Self >) -> usize {
             self . index_property () }
         fn parent_node (self : :: core :: pin :: Pin < & Self > , _result : & mut sp :: ItemWeak) {
             }
         fn embed_component (self : :: core :: pin :: Pin < & Self > , _parent_component : & sp :: ComponentWeak , _item_tree_index : usize) -> bool {
             false }
         fn layout_info (self : :: core :: pin :: Pin < & Self > , orientation : sp :: Orientation) -> sp :: LayoutInfo {
             self . layout_info (orientation) }
         fn accessible_role (self : :: core :: pin :: Pin < & Self > , index : usize) -> sp :: AccessibleRole {
             self . accessible_role (index) }
         fn accessible_string_property (self : :: core :: pin :: Pin < & Self > , index : usize , what : sp :: AccessibleStringProperty , result : & mut sp :: SharedString ,) {
             * result = self . accessible_string_property (index , what) ;
             }
         fn window_adapter (self : :: core :: pin :: Pin < & Self > , do_create : bool , result : & mut Option < Rc < dyn WindowAdapter >> ,) {
             if do_create {
                 * result = Some (self . window_adapter_impl ()) ;
                 }
             else {
                 * result = self . maybe_window_adapter_impl () ;
                 }
             }
         }
     pub struct r#MainWindow (vtable :: VRc < sp :: ComponentVTable , InnerMainWindow >) ;
     impl r#MainWindow {
         pub fn new () -> core :: result :: Result < Self , slint :: PlatformError > {
             let inner = InnerMainWindow :: new () ? ;
             inner . globals . global_ColorSchemeSelector_37 . clone () . init (& inner) ;
             inner . globals . global_Palette_38 . clone () . init (& inner) ;
             InnerMainWindow :: user_init (sp :: VRc :: map (inner . clone () , | x | x)) ;
             core :: result :: Result :: Ok (Self (inner)) }
         # [allow (dead_code)] pub fn invoke_connect_clicked (& self , arg_0 : sp :: SharedString ,) -> () {
             let _self = vtable :: VRc :: as_pin_ref (& self . 0) ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_connect_clicked }
            ) . apply_pin (_self) . call (& (arg_0 ,)) }
         # [allow (dead_code)] pub fn on_connect_clicked (& self , mut f : impl FnMut (sp :: SharedString) -> () + 'static) {
             let _self = vtable :: VRc :: as_pin_ref (& self . 0) ;
             # [allow (unused)] ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_connect_clicked }
            ) . apply_pin (_self) . set_handler (move | args | f (args . 0 . clone ())) }
         # [allow (dead_code)] pub fn get_connection_status (& self) -> sp :: SharedString {
             # [allow (unused_imports)] let _self = vtable :: VRc :: as_pin_ref (& self . 0) ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_connection_status }
            ) . apply_pin (_self) . get () }
         # [allow (dead_code)] pub fn set_connection_status (& self , value : sp :: SharedString) {
             # [allow (unused_imports)] let _self = vtable :: VRc :: as_pin_ref (& self . 0) ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_connection_status }
            ) . apply_pin (_self) . set (value as _) }
         # [allow (dead_code)] pub fn get_error_message (& self) -> sp :: SharedString {
             # [allow (unused_imports)] let _self = vtable :: VRc :: as_pin_ref (& self . 0) ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_error_message }
            ) . apply_pin (_self) . get () }
         # [allow (dead_code)] pub fn set_error_message (& self , value : sp :: SharedString) {
             # [allow (unused_imports)] let _self = vtable :: VRc :: as_pin_ref (& self . 0) ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_error_message }
            ) . apply_pin (_self) . set (value as _) }
         # [allow (dead_code)] pub fn get_is_connected (& self) -> bool {
             # [allow (unused_imports)] let _self = vtable :: VRc :: as_pin_ref (& self . 0) ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_is_connected }
            ) . apply_pin (_self) . get () }
         # [allow (dead_code)] pub fn set_is_connected (& self , value : bool) {
             # [allow (unused_imports)] let _self = vtable :: VRc :: as_pin_ref (& self . 0) ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_is_connected }
            ) . apply_pin (_self) . set (value as _) }
         }
     impl From < r#MainWindow > for vtable :: VRc < sp :: ComponentVTable , InnerMainWindow > {
         fn from (value : r#MainWindow) -> Self {
             value . 0 }
         }
     impl slint :: ComponentHandle for r#MainWindow {
         type Inner = InnerMainWindow ;
         fn as_weak (& self) -> slint :: Weak < Self > {
             slint :: Weak :: new (& self . 0) }
         fn clone_strong (& self) -> Self {
             Self (self . 0 . clone ()) }
         fn from_inner (inner : vtable :: VRc < sp :: ComponentVTable , InnerMainWindow >) -> Self {
             Self (inner) }
         fn run (& self) -> core :: result :: Result < () , slint :: PlatformError > {
             self . show () ? ;
             slint :: run_event_loop () ? ;
             self . hide () ? ;
             core :: result :: Result :: Ok (()) }
         fn show (& self) -> core :: result :: Result < () , slint :: PlatformError > {
             self . 0 . window_adapter_ref () ? . window () . show () }
         fn hide (& self) -> core :: result :: Result < () , slint :: PlatformError > {
             self . 0 . window_adapter_ref () ? . window () . hide () }
         fn window (& self) -> & slint :: Window {
             self . 0 . window_adapter_ref () . unwrap () . window () }
         fn global < 'a , T : slint :: Global < 'a , Self >> (& 'a self) -> T {
             T :: get (& self) }
         }
     # [allow (dead_code)] struct Globals_MainWindow {
         global_ColorSchemeSelector_37 : :: core :: pin :: Pin < sp :: Rc < InnerColorSchemeSelector_37 >> , global_Palette_38 : :: core :: pin :: Pin < sp :: Rc < InnerPalette_38 >> , }
     impl Default for Globals_MainWindow {
         fn default () -> Self {
             Self {
                 global_ColorSchemeSelector_37 : InnerColorSchemeSelector_37 :: new () , global_Palette_38 : InnerPalette_38 :: new () , }
             }
         }
     const _THE_SAME_VERSION_MUST_BE_USED_FOR_THE_COMPILER_AND_THE_RUNTIME : slint :: VersionCheck_1_2_0 = slint :: VersionCheck_1_2_0 ;
     }
 pub use slint_generatedMainWindow :: {
     r#MainWindow , r#TextStyle }
 ;
 pub use slint :: {
     ComponentHandle as _ , Global as _ , ModelExt as _ }
 ;
