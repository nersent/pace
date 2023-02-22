import os
from os import path
import numpy as np
import pandas as pd
import matplotlib.pyplot as plt



# data_path = path.join(".data")
data_path = path.join(".", "data" + os.sep)
metrics_path = path.join(data_path, "metrics.parquet")
net_equity_path = path.join(data_path, "net_equity.parquet")


def normal_score(value, min_value, max_value):
    score = 0 if value < min_value else (np.log(value) - np.log(min_value)) / (np.log(max_value) - np.log(min_value))
    score = max(0, min(score, 1))
    return score


def max_dd_score(max_dd):
    
    if max_dd < 20 :
        max_dd_score = 1
    elif (max_dd >= 20) and (max_dd < 25) :
        max_dd_score = 0.9
    elif (max_dd >= 25) and (max_dd < 30) :
        max_dd_score = 0.8
    elif (max_dd >= 30) and (max_dd < 35) :
        max_dd_score = 0.6
    elif (max_dd >= 35) and (max_dd < 40) :
        max_dd_score = 0.4
    elif (max_dd >= 40) and (max_dd < 45) :
        max_dd_score = 0.2
    elif (max_dd >= 45) and (max_dd < 50) :
        max_dd_score = 0.1
    else :
        max_dd_score = 0
        
    return max_dd_score


def min_max_score(value,min_value,max_value):
    if value <= min_value:
        return  0
    elif value >= max_value:
        return  0
    else :
        return 1
    

def Combined_score(metrics):
    
    net_profit_min = 100
    net_profit_max = 10000
    net_profit_weight = 0.25


    Sharpe_min = 0.5
    Sharpe_max = 3
    Sharpe_weight = 0.25


    Omega_min = 20
    Omega_max = 35
    Omega_weight = 0.25


    max_dd_weight = 0.25
    
    trades_min_value = 10
    trades_max_value = 100
    
    
    net_profit = metrics['net_profit_percent']
    Sharpe = metrics['equity_sharpe_ratio']
    Omega = metrics['equity_omega_ratio']
    max_dd = metrics['equity_max_drawdown_percent'] * (-1) * (100)
    trades = metrics['closed_trades']
    
    
    net_profit_normal_score = normal_score(net_profit, net_profit_min, net_profit_max)
    net_profit_weighted_score = net_profit_normal_score * net_profit_weight
    
    Sharpe_normal_score = normal_score(Sharpe, Sharpe_min, Sharpe_max)
    Sharpe_weighted_score = Sharpe_normal_score * Sharpe_weight
    
    Omega_normal_score = normal_score(Omega, Omega_min, Omega_max)
    Omega_weighted_score = Omega_normal_score * Omega_weight
    
    max_dd_normal_score = max_dd_score(max_dd)
    max_dd_weighted_score = max_dd_normal_score * max_dd_weight
    
    trades_score = min_max_score(trades, trades_min_value, trades_max_value)
    
    
    if min(net_profit_weighted_score,Sharpe_weighted_score,Omega_weighted_score,max_dd_weighted_score,trades_score) == 0:
        result = 0
    else :
        result = net_profit_weighted_score + Sharpe_weighted_score + Omega_weighted_score + max_dd_weighted_score
        
        
    return result


def view(metrics: dict, net_equity_history: list[float]):
    id = int(metrics["id"])

    print(f'{metrics}\n\n')

    net_equity = metrics["net_equity"]
    profit_factor = metrics["profit_factor"]
    equity_omega_ratio = metrics["equity_omega_ratio"]
    equity_sharpe_ratio = metrics["equity_sharpe_ratio"]
    equity_sortino_ratio = metrics["equity_sortino_ratio"]
    net_equity_omega_ratio = metrics["net_equity_omega_ratio"]
    net_equity_sharpe_ratio = metrics["net_equity_sharpe_ratio"]
    net_equity_sortino_ratio = metrics["net_equity_sortino_ratio"]
    equity_max_drawdown_percent = metrics["equity_max_drawdown_percent"]
    intra_trade_max_drawdown_percent = metrics["intra_trade_max_drawdown_percent"]
    max_drawdown_percent = metrics["max_drawdown_percent"]

    print(f"ID: {id}")
    print(f"Net Equity: {net_equity}")
    print(f"Profit Factor: {profit_factor}")
    print(f"Net Equity Omega Ratio: {net_equity_omega_ratio}")
    print(f"Net Equity Sharpe Ratio: {net_equity_sharpe_ratio}")
    print(f"Net Equity Sortino Ratio: {net_equity_sortino_ratio}")
    print(f"Equity Omega Ratio: {equity_omega_ratio}")
    print(f"Equity Sharpe Ratio: {equity_sharpe_ratio}")
    print(f"Equity Sortino Ratio: {equity_sortino_ratio}")
    print(f"Equity Max Drawdown Percent: {equity_max_drawdown_percent}")
    print(
        f"Intra Trade Max Drawdown Percent: {intra_trade_max_drawdown_percent}")
    print(f"Max Drawdown Percent: {max_drawdown_percent}")

    plt.title(f"Net Equity History for ID {id}")
    plt.xlabel("Trade")
    plt.ylabel("Net Equity")
    plt.plot(net_equity_history)
    plt.xticks(range(1, len(net_equity_history)))
    plt.show()

    

if __name__ == "__main__":

    df_metrics = pd.read_parquet(metrics_path)
    df_net_equity = pd.read_parquet(net_equity_path)

    highest_score = df_metrics.apply(Combined_score, axis=1)
    
    if highest_score.max() == 0 :
        print("No good results")
    
    else :
        best_label = highest_score.idxmax()
        best_metrics = df_metrics.loc[best_label].to_dict()
        best_id = int(best_metrics["id"])
        best_net_equity_history: list[float] = df_net_equity[df_net_equity["id"]
                                                             == best_id]["net_equity"].values
            
        view(best_metrics, best_net_equity_history)