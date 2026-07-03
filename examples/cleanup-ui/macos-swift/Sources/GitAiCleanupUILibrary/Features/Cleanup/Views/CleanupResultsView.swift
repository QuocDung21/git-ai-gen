import AppKit
import SwiftUI

struct CleanupResultsView: View {
    let items: [CleanupItem]
    @Binding var selected: Set<String>
    @Binding var expandedGroups: Set<String>

    var body: some View {
        VStack(spacing: 0) {
            header
            Divider()

            ScrollView {
                LazyVStack(spacing: 0) {
                    ForEach(groups) { group in
                        groupRow(group)

                        if expandedGroups.contains(group.id) {
                            ForEach(group.items) { item in
                                itemRow(item)
                            }
                        }
                    }
                }
            }
        }
        .frame(minWidth: 560)
    }

    private var header: some View {
        HStack {
            Text("Item")
                .fontWeight(.semibold)
            Spacer()
            Text("Size")
                .fontWeight(.semibold)
                .frame(width: 120, alignment: .trailing)
        }
        .font(.callout)
        .padding(.horizontal, 14)
        .padding(.vertical, 10)
        .background(Color(nsColor: .controlBackgroundColor))
    }

    private var groups: [CleanupItemGroup] {
        let grouped = Dictionary(grouping: items, by: \.groupName)
        return grouped
            .map { CleanupItemGroup(name: $0.key, items: $0.value.sorted { $0.path < $1.path }) }
            .sorted {
                if $0.sizeBytes == $1.sizeBytes {
                    return $0.name < $1.name
                }
                return $0.sizeBytes > $1.sizeBytes
            }
    }

    private func groupRow(_ group: CleanupItemGroup) -> some View {
        HStack(spacing: 8) {
            Button {
                toggleExpanded(group)
            } label: {
                Image(systemName: expandedGroups.contains(group.id) ? "chevron.down" : "chevron.right")
                    .frame(width: 14)
            }
            .buttonStyle(.plain)

            Button {
                toggleGroupSelection(group)
            } label: {
                Image(systemName: groupSelectionIcon(group))
                    .foregroundStyle(.secondary)
                    .frame(width: 18)
            }
            .buttonStyle(.plain)

            Text(group.name)
                .fontWeight(.semibold)

            Text("\(group.items.count)")
                .font(.caption)
                .foregroundStyle(.secondary)
                .padding(.horizontal, 6)
                .padding(.vertical, 2)
                .background(Color(nsColor: .quaternaryLabelColor))
                .clipShape(RoundedRectangle(cornerRadius: 5))

            Spacer()

            Text(group.formattedSize)
                .fontWeight(.semibold)
                .foregroundStyle(.secondary)
                .frame(width: 120, alignment: .trailing)
        }
        .padding(.horizontal, 12)
        .padding(.vertical, 8)
        .background(Color(nsColor: .textBackgroundColor))
    }

    private func itemRow(_ item: CleanupItem) -> some View {
        HStack(spacing: 8) {
            Spacer()
                .frame(width: 22)

            Button {
                toggleItemSelection(item)
            } label: {
                Image(systemName: selected.contains(item.id) ? "checkmark.square.fill" : "square")
                    .foregroundColor(selected.contains(item.id) ? .accentColor : .secondary)
                    .frame(width: 18)
            }
            .buttonStyle(.plain)

            VStack(alignment: .leading, spacing: 3) {
                Text(item.displayName)
                    .fontWeight(.medium)
                    .lineLimit(1)

                Text(item.path)
                    .font(.caption)
                    .foregroundStyle(.secondary)
                    .lineLimit(1)
                    .truncationMode(.middle)
            }

            Spacer()

            Text(item.formattedSize)
                .foregroundStyle(.secondary)
                .frame(width: 120, alignment: .trailing)

            Button {
                showInFinder(item)
            } label: {
                Image(systemName: "folder")
                    .frame(width: 20)
            }
            .buttonStyle(.plain)
            .help("Show in Finder")
        }
        .padding(.horizontal, 12)
        .padding(.vertical, 7)
        .background(Color(nsColor: .windowBackgroundColor))
        .contextMenu {
            Button("Show in Finder") {
                showInFinder(item)
            }
        }
    }

    private func toggleExpanded(_ group: CleanupItemGroup) {
        if expandedGroups.contains(group.id) {
            expandedGroups.remove(group.id)
        } else {
            expandedGroups.insert(group.id)
        }
    }

    private func toggleItemSelection(_ item: CleanupItem) {
        if selected.contains(item.id) {
            selected.remove(item.id)
        } else {
            selected.insert(item.id)
        }
    }

    private func toggleGroupSelection(_ group: CleanupItemGroup) {
        let ids = Set(group.items.map(\.id))
        if ids.isSubset(of: selected) {
            selected.subtract(ids)
        } else {
            selected.formUnion(ids)
        }
    }

    private func groupSelectionIcon(_ group: CleanupItemGroup) -> String {
        let selectedCount = group.items.filter { selected.contains($0.id) }.count
        if selectedCount == group.items.count {
            return "checkmark.square.fill"
        }
        if selectedCount > 0 {
            return "minus.square.fill"
        }
        return "square"
    }

    private func showInFinder(_ item: CleanupItem) {
        NSWorkspace.shared.activateFileViewerSelecting([
            URL(fileURLWithPath: item.path)
        ])
    }
}
