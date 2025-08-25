# /// script
# requires-python = ">=3.12"
# dependencies = [
#     "matplotlib",
#     "polars",
# ]
# ///

from matplotlib import pyplot as plt
import polars as pl


def main() -> None:
    df = pl.read_csv("data/dls_resource_table.csv")
    print(df.head())
    # breakpoint()
    rename_expressions = {f"{x} Wkts": f"{10-x}_wl" for x in range(10)}
    df = (
        df.with_columns(
            (50 - pl.col("Overs Left")).alias("overs_played"),
        )
        .rename(rename_expressions)
        .drop("Overs Left")
    )
    print(df.head())

    # let's make a linestyle map that styles the lines according
    # to how many wickets are left

    line_styles = [":", "--", "-"]

    fig, ax = plt.subplots(1, 1)
    for x in range(1, 11):
        linestyle = line_styles[min(x // 3, 2)]
        linewidth = 0.8 + (0.1 * x)

        ax.plot(
            df["overs_played"],
            df[f"{x}_wl"],
            linewidth=linewidth,
            color="orange",
            linestyle=linestyle,
        )

        text = f"{x}"
        if x < 4:
            text = text + " wk left"
        ax.text(0, df[f"{x}_wl"].first(), text, ha="left", va="bottom", fontsize=10)
    ax.set_ylabel("Resource Left")
    ax.set_xlabel("Overs Played")
    ax.set_title("DLS System Resource Graph")
    ax.grid()
    plt.show()


if __name__ == "__main__":
    main()
