module Main exposing (..)

import Html exposing (Html, text, div, h1, img)
import Json.Decode as Decode
import Array exposing (Array)
import Html.Attributes exposing (src)
import Http


---- MODEL ----


type alias Model =
    { slaps : Array String
    , slapIdx : Int
    }


init : ( Model, Cmd Msg )
init =
    ( { slaps = Array.empty, slapIdx = 0 }, Http.send SlapsReceived (Http.get "http://localhost:8080/cat/slaps" slapsDecoder) )


slapsDecoder : Decode.Decoder (Array String)
slapsDecoder =
    Decode.array Decode.string



---- UPDATE ----


type Msg
    = GetSlaps
    | SlapsReceived (Result Http.Error (Array String))
    | NextSlap


update : Msg -> Model -> ( Model, Cmd Msg )
update msg model =
    case msg of
        NextSlap ->
            ( { model | slapIdx = model.slapIdx + 1 }, Cmd.none )

        GetSlaps ->
            ( model, Cmd.none )

        SlapsReceived res ->
            case res of
                Err _ ->
                    ( model, Cmd.none )

                Ok arr ->
                    ( { model | slaps = arr }, Cmd.none )



---- VIEW ----


view : Model -> Html Msg
view model =
    div []
        [ h1 [] [ text "CAT. SLAPP!!!" ]
        , content model
        ]


content : Model -> Html Msg
content model =
    case Array.get model.slapIdx model.slaps of
        Just slap ->
            div []
                [ img [ src slap ] []
                ]

        Nothing ->
            div [] [ text "loading..." ]



---- PROGRAM ----


main : Program Never Model Msg
main =
    Html.program
        { view = view
        , init = init
        , update = update
        , subscriptions = always Sub.none
        }
