use topcoat::{context::Cx, router::request::uri};
use topcoat_ant_design::UiLanguage;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum Locale {
    En,
    Zh,
}

impl Locale {
    pub(crate) fn current(cx: &Cx) -> Self {
        if uri(cx)
            .query()
            .is_some_and(|query| query.split('&').any(|pair| pair == "lang=zh"))
        {
            Self::Zh
        } else {
            Self::En
        }
    }

    pub(crate) fn html_lang(self) -> &'static str {
        match self {
            Self::En => "en",
            Self::Zh => "zh-CN",
        }
    }

    pub(crate) fn select<'a>(self, en: &'a str, zh: &'a str) -> &'a str {
        match self {
            Self::En => en,
            Self::Zh => zh,
        }
    }

    pub(crate) fn ui(self) -> UiLanguage {
        match self {
            Self::En => UiLanguage::English,
            Self::Zh => UiLanguage::ChineseSimplified,
        }
    }

    pub(crate) fn link(self, path: &str) -> String {
        match self {
            Self::En => path.to_owned(),
            Self::Zh => format!("{path}?lang=zh"),
        }
    }
}

pub(crate) fn text(locale: Locale, chinese: &'static str) -> &'static str {
    if locale == Locale::Zh {
        return chinese;
    }

    match chinese {
        "Accordion 手风琴" => "Accordion",
        "Collapse 折叠动画" => "Collapse",
        "DateTimeRange 时间范围" => "DateTimeRange",
        "Dialog 模态对话框" => "Dialog",
        "Drawer 抽屉" => "Drawer",
        "FormField 表单字段" => "FormField",
        "GitLab 实例" => "GitLab instance",
        "GitLab 实例“研发主站”已验证。" => {
            "GitLab instance “Main development” was verified."
        }
        "GitLab 项目" => "GitLab projects",
        "GitLab 项目示例" => "GitLab project example",
        "Icons 图标" => "Icons",
        "LDAP 配置" => "LDAP configuration",
        "Merge Request 评论" => "Merge Request comments",
        "Notification 通知提醒框" => "Notification",
        "Popconfirm 气泡确认框" => "Popconfirm",
        "Table 数据表格" => "Table",
        "Tabs 路由页签" => "Tabs",
        "Tag 标签" => "Tag",
        "Token 将在 7 天后到期，请及时更新。" => {
            "The token expires in 7 days. Update it soon."
        }
        "Tooltip 文字提示" => "Tooltip",
        "Webhook 事件详情" => "Webhook event details",
        "Webhook 配置" => "Webhook configuration",
        "三个显式接入点" => "Three explicit integration points",
        "上一页" => "Previous",
        "下一页" => "Next",
        "为未知高度内容提供可逆的展开与收起过渡。" => {
            "A reversible transition for content of unknown height."
        }
        "事件信息" => "Event details",
        "事件类型" => "Event type",
        "事件记录" => "Event history",
        "以文字和语义颜色区分状态。" => {
            "Distinguish status with text and semantic color."
        }
        "使用 Topcoat component、signal 和响应式属性，为未知高度内容提供平滑、可逆的展开与收起。" => {
            "Use Topcoat components, signals, and reactive attributes for a smooth, reversible transition."
        }
        "使用原生模态语义承载表单和集中操作。" => {
            "Use native modal semantics for forms and focused tasks."
        }
        "使用原生表格语义展示结构化数据，并组合 Topcoat signal 或真实链接完成分页。" => {
            "Display structured data with native table semantics and paginate with Topcoat signals or real links."
        }
        "使用真实链接组织同一对象下的多个页面，并由 Topcoat 路由决定当前状态。" => {
            "Organize related pages with real links and let the Topcoat route determine the active tab."
        }
        "页签在浏览器中即时切换；直接打开某个页签地址时，会显示对应的初始内容。" => {
            "Tabs switch instantly in the browser; opening a tab URL directly selects its initial panel."
        }
        "使用组件库" => "Use the component library",
        "例如 Merge Request 变化或分支 Push。" => {
            "For example, a Merge Request change or a branch push."
        }
        "保存" => "Save",
        "保存、验证和启用成功。" => "Saved, verified, and enabled successfully.",
        "信息状态" => "Information status",
        "修改连接信息后保存。这里仅演示组件，不会发送请求。" => {
            "Edit and save connection details. This demo does not send a request."
        }
        "停用" => "Disable",
        "停用后将不再处理该实例的 Webhook。" => {
            "This instance will stop processing webhooks."
        }
        "停用实例" => "Disable instance",
        "先完成五步接入" => "Complete the five integration steps",
        "共 4 个项目" => "4 projects",
        "关闭后内容不可见且不会响应鼠标。" => {
            "Closed content is invisible and does not respond to pointer events."
        }
        "内容高度无需预先计算。Grid 轨道会从 0fr 过渡到 1fr，关闭时按同一路径反向执行。" => {
            "No height measurement is needed. The grid track moves from 0fr to 1fr and reverses on close."
        }
        "内部操作" => "Inner action",
        "切换页签后，路由、激活态和下方内容会一起更新。" => {
            "Switching tabs updates the route, active state, and content together."
        }
        "点击页签即可切换内容，无需刷新页面。" => {
            "Switch panels without reloading the page."
        }
        "刷新" => "Refresh",
        "加载页面资源" => "Load page assets",
        "动效" => "Motion",
        "危险操作" => "Destructive action",
        "危险样式由调用方的确认按钮决定。" => {
            "The caller chooses the confirm button's destructive styling."
        }
        "原生模态行为包含焦点约束、Escape 关闭和统一标题栏。" => {
            "Native modal behavior includes focus management, Escape to close, and a consistent heading."
        }
        "反馈" => "Feedback",
        "取消" => "Cancel",
        "可以连续快速点击，观察动画从当前位置自然反向。" => {
            "Click quickly several times to see the animation reverse from its current position."
        }
        "右侧提示" => "Right tooltip",
        "右侧空间不足时，提示向左偏移，箭头仍然指向触发元素。" => {
            "Near the right edge, the tooltip shifts left while its arrow keeps pointing at the trigger."
        }
        "启用" => "Enable",
        "启用前仍会验证当前 Token。" => {
            "The current token will be verified before enabling."
        }
        "启用实例" => "Enable instance",
        "图标继承文字颜色，默认跟随字号缩放；下列项目均直接使用 Topcoat icon 组件渲染。" => {
            "Icons inherit text color and scale with the font by default. These are rendered with Topcoat's icon component."
        }
        "在不离开列表的情况下查看一条记录的完整详情。" => {
            "View complete record details without leaving the list."
        }
        "在弹出层中选择开始和结束时间。" => {
            "Choose a start and end time in a popover."
        }
        "在当前列表上方查看完整记录详情。" => {
            "View full record details above the current list."
        }
        "在视口右上角反馈操作结果，支持四种语义状态。" => {
            "Show operation results in the upper-right corner with four semantic tones."
        }
        "基础字段" => "Basic field",
        "基础折叠" => "Basic collapse",
        "处理成功" => "Processed successfully",
        "处理状态" => "Processing status",
        "多个区块共用展开状态；切换时自动收起另一组，折叠中的表单值保持不变。" => {
            "Sections share open state. Switching closes the previous section while preserving form values."
        }
        "多段内容" => "Multiple content sections",
        "失败" => "Failed",
        "审查任务" => "Review tasks",
        "审计日志" => "Audit log",
        "导航" => "Navigation",
        "语言" => "Language",
        "尚无事件" => "No events yet",
        "工作区引用 path，外部项目固定 Git commit。" => {
            "Use a path dependency in a workspace or pin a Git commit externally."
        }
        "左侧提示" => "Left tooltip",
        "左侧空间不足时，提示向右偏移，箭头仍然指向触发元素。" => {
            "Near the left edge, the tooltip shifts right while its arrow keeps pointing at the trigger."
        }
        "已停用" => "Disabled",
        "已启用" => "Enabled",
        "已生效" => "Active",
        "开始" => "Get started",
        "待确认" => "Pending confirmation",
        "待验证" => "Pending verification",
        "快速开始" => "Quick start",
        "悬停或聚焦查看完整文本，气泡与箭头自动适应视口边缘。" => {
            "Hover or focus to see the full text. The bubble and arrow adapt near viewport edges."
        }
        "成功状态" => "Success status",
        "所属实例" => "Instance",
        "打开后可点击遮罩、关闭按钮或按 Escape。" => {
            "Close with the backdrop, close button, or Escape."
        }
        "打开快速开始 →" => "Open quick start →",
        "把 Topcoat Ant Design 接入 Topcoat 应用，需要完成依赖、页面资源和路由注册，再按需调用组件。" => {
            "Integrate Topcoat Ant Design by adding the dependency, loading page assets, registering routes, and using components."
        }
        "把两个日期时间字段收纳为紧凑的弹出筛选控件，并保留原生表单提交能力。" => {
            "Keep two date-time fields in a compact popover while retaining native form submission."
        }
        "指令" => "Command",
        "指令只能包含小写字母、数字、连字符和下划线。" => {
            "Use lowercase letters, digits, hyphens, or underscores."
        }
        "接入流程" => "Integration flow",
        "控制台图标集" => "Console icon set",
        "搜索" => "Search",
        "操作失败" => "Operation failed",
        "操作成功" => "Operation succeeded",
        "操作提示" => "Operation notice",
        "支持快捷范围、清除和应用；示例不会提交到业务接口。" => {
            "Use preset ranges, clear, and apply. This demo does not submit to a business API."
        }
        "收起示例代码" => "Hide example code",
        "数据展示" => "Data display",
        "数据录入" => "Data entry",
        "无法连接 GitLab，请检查地址和 Token。" => {
            "Could not connect to GitLab. Check the URL and token."
        }
        "无需复制 CSS" => "No CSS copying",
        "时间范围筛选" => "Date-time filter",
        "显示示例代码" => "Show example code",
        "显示通知" => "Show notification",
        "普通操作" => "Regular action",
        "普通状态和操作说明。" => "General status and operation details.",
        "最近事件" => "Latest event",
        "服务端校验失败时用统一位置呈现可操作的字段错误。" => {
            "Show actionable field errors consistently when server validation fails."
        }
        "未展开的分组不会清除选择状态。" => {
            "Closing a section does not clear its selections."
        }
        "未配置" => "Not configured",
        "查看不同语义颜色的标签。" => "Compare tags with different semantic colors.",
        "查看事件详情" => "View event details",
        "查看依赖、页面资源、AssetBundle、Router 和第一个组件的完整示例。" => {
            "See a complete example covering dependencies, page assets, AssetBundle, Router, and your first component."
        }
        "查看组件 →" => "View component →",
        "查看该项目收到的 Webhook 事件、支持状态和处理结果。" => {
            "Review received webhooks, support status, and processing results for this project."
        }
        "标签会通过稳定 id 与输入控件关联；必填标记只表达界面语义。" => {
            "A stable ID associates the label with its input; the required marker expresses UI semantics."
        }
        "根布局调用 head_assets() 加载样式和字体。" => {
            "Call head_assets() in the root layout to load CSS and fonts."
        }
        "每页记录" => "Rows per page",
        "气泡靠近视口边缘时保持可见，箭头继续指向触发按钮。" => {
            "The bubble stays visible near viewport edges while its arrow points to the trigger."
        }
        "注册服务端资源" => "Register server assets",
        "添加依赖" => "Add dependency",
        "点击分页按钮，行内容和禁用状态由 Topcoat signal 在浏览器中同步更新。" => {
            "Pagination buttons update rows and disabled states through Topcoat signals in the browser."
        }
        "点击按钮查看不同语义的通知。" => {
            "Click the buttons to see notifications with different tones."
        }
        "状态" => "Status",
        "独立浏览每个组件的真实样式、Topcoat 交互和共用的 API 文档。" => {
            "Explore each component's real styles, Topcoat interactions, and shared API documentation."
        }
        "用于上下文明确的轻量二次确认。下方说明与 popconfirm 方法的 Rustdoc 来自同一份 Markdown。" => {
            "A lightweight confirmation for contextual actions. The guide below is shared with the popconfirm Rustdoc."
        }
        "用于保存、验证和同步等操作反馈。下方说明与 notification 方法的 Rustdoc 来自同一份 Markdown。" => {
            "Feedback for saving, validation, and synchronization. The guide below is shared with the notification Rustdoc."
        }
        "用于编辑连接、填写配置等需要集中处理的任务。下方说明与 dialog 方法的 Rustdoc 来自同一份 Markdown。" => {
            "Focused tasks such as editing connections and configuration. The guide below is shared with the dialog Rustdoc."
        }
        "用于项目内识别这项自动化。" => {
            "Used to identify this automation in the project."
        }
        "用户与权限" => "Users and permissions",
        "用真实路由组织对象详情页面。" => "Use real routes for detail pages.",
        "用统一密度展示数据，并组合页码或游标分页。" => {
            "Present data consistently with numbered or cursor pagination."
        }
        "由 Topcoat Iconify 在编译期校验并生成的 Ant Design 内联 SVG 图标。" => {
            "Ant Design inline SVG icons validated and generated by Topcoat Iconify at build time."
        }
        "由宿主 Router 暴露字体与静态资源。" => {
            "Serve fonts and static assets through the host Router."
        }
        "确认" => "Confirm",
        "确认停用此实例？" => "Disable this instance?",
        "确认后气泡立即恢复隐藏状态，业务回调继续执行。" => {
            "Confirming hides the bubble immediately, then runs the caller's action."
        }
        "确认启用此实例？" => "Enable this instance?",
        "确认当前用户是否有权查看项目配置和事件请求正文。" => {
            "Check whether this user can view project settings and event request bodies."
        }
        "确认执行边缘操作？" => "Run the edge-case action?",
        "示例 GitLab" => "Example GitLab",
        "示例代码" => "Example code",
        "示例实例已停用。" => "Example instance disabled.",
        "示例实例已启用。" => "Example instance enabled.",
        "符合条件的事件直接创建任务" => "Matching events create tasks directly",
        "第一次使用" => "First time here",
        "筛选" => "Filter",
        "管理员" => "Admin",
        "系统减少动态效果时自动取消过渡。" => {
            "The transition is disabled when reduced motion is requested."
        }
        "紧凑表格与页码分页" => "Compact table with numbered pagination",
        "组件导航" => "Component navigation",
        "组件概览" => "Component overview",
        "组件示例分页" => "Example pagination",
        "组件预览" => "Component preview",
        "统一标签、辅助信息和错误反馈，同时保留原生输入控件与 Topcoat 绑定能力。" => {
            "Unify labels, hints, and errors while retaining native inputs and Topcoat bindings."
        }
        "统一表单字段标签、说明与错误反馈。" => {
            "Consistent field labels, help, and errors."
        }
        "编辑示例连接" => "Edit example connection",
        "编辑连接" => "Edit connection",
        "缩窄窗口或滚动页面可以观察偏移与翻转。" => {
            "Narrow the window or scroll to see shifting and flipping."
        }
        "自动事件 Trigger" => "Automatic event trigger",
        "自动化名称" => "Automation name",
        "菜单" => "Menu",
        "角色" => "Role",
        "警告状态" => "Warning status",
        "设置回调地址和 Secret，并检查最近一次事件是否验证成功。" => {
            "Set the callback URL and secret, then check validation of the latest event."
        }
        "访问权限" => "Access permissions",
        "评论 Trigger" => "Comment trigger",
        "试着切换两组，并勾选评论入口观察标题计数。" => {
            "Switch between the two sections and check a comment entry to see the heading count change."
        }
        "请注意" => "Attention",
        "账号" => "Account",
        "贴近操作入口完成轻量确认，并处理边缘偏移和翻转。" => {
            "Confirm actions beside their triggers, with edge shifting and flipping."
        }
        "跳转" => "Navigate",
        "边缘偏移" => "Edge shifting",
        "边缘定位验证完成。" => "Edge positioning verified.",
        "边缘操作" => "Edge action",
        "运行中" => "Running",
        "运行总览" => "Overview",
        "连接名称" => "Connection name",
        "连通性测试" => "Connectivity test",
        "适合单元格包含说明文字或操作入口的管理表格。" => {
            "For admin tables whose cells contain supporting text or actions."
        }
        "适用于启用、重新验证等可恢复操作。" => {
            "For reversible actions such as enabling or revalidating."
        }
        "选择由评论指令触发的入口" => {
            "Choose entry points triggered by comment commands"
        }
        "透明度与高度同步过渡。" => "Opacity and height transition together.",
        "错误状态" => "Error state",
        "需要注意但仍可继续。" => "Needs attention, but you can continue.",
        "需要立即关注的操作失败。" => {
            "An operation failed and needs immediate attention."
        }
        "项目" => "Project",
        "项目列表已经同步到最新状态。" => "The project list is up to date.",
        "项目详情示例" => "Project details example",
        "默认密度" => "Default density",
        "默认密度示例" => "Default density example",
        "鼠标移入提示可选择文本；Tab 聚焦也可显示，Escape 关闭。滚动页面或缩窄窗口可验证翻转和箭头偏移。" => {
            "Hover to select tooltip text, focus with Tab, or close with Escape. Scroll or resize to see edge positioning."
        }
        other => other,
    }
}
