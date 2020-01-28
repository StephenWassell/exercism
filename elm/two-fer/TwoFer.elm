module TwoFer exposing (twoFer)


twoFer : Maybe String -> String
twoFer name =
    case name of
        Just s ->
            "One for " ++ s ++ ", one for me."

        Nothing ->
            twoFer (Just "you")
